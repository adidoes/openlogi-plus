//! Foreground-application watcher (P1.4).
//!
//! Polls [`openlogi_hook::frontmost_bundle_id`] once per second on a dedicated
//! OS thread and forwards transitions over an unbounded mpsc. Steady-state
//! apps don't send updates — the channel only fires when the value changes,
//! so the GPUI thread doesn't wake up unnecessarily.
//!
//! Non-macOS platforms compile to a stub: the sender is dropped immediately
//! so the receiver closes on first `recv().await` and the GUI loop falls
//! through to its idle path.

use std::thread;
use std::time::Duration;

use tokio::sync::mpsc;
use tracing::{debug, warn};

/// Channel item: `Some(bundle_id)` when an app is frontmost; `None` for
/// "no foreground app" (rare on macOS — Finder is usually frontmost even
/// when nothing else is).
pub type ForegroundUpdate = Option<String>;

/// Start the watcher and return a receiver of bundle-id transitions. The
/// initial value is also pushed so consumers don't need a separate query.
///
/// Dropping the receiver shuts the watcher down: the next `send` fails and
/// the loop exits cleanly.
pub fn spawn(period: Duration) -> mpsc::UnboundedReceiver<ForegroundUpdate> {
    let (tx, rx) = mpsc::unbounded_channel();
    if !cfg!(target_os = "macos") {
        // No backend on this platform — drop the sender so the receiver
        // closes immediately and the GUI loop's `while let Some(..)`
        // falls through.
        drop(tx);
        let _ = period;
        return rx;
    }
    let spawn_result = thread::Builder::new()
        .name("openlogi-app-watcher".into())
        .spawn(move || {
            let mut last: ForegroundUpdate = None;
            let mut first_tick = true;
            loop {
                let current = openlogi_hook::frontmost_bundle_id();
                if first_tick || current != last {
                    debug!(?current, ?last, "frontmost app changed");
                    if tx.send(current.clone()).is_err() {
                        debug!("app watcher receiver dropped — exiting");
                        return;
                    }
                    last = current;
                    first_tick = false;
                }
                thread::sleep(period);
            }
        });
    if let Err(e) = spawn_result {
        warn!(error = %e, "could not spawn app watcher — per-app profiles disabled");
    }
    rx
}
