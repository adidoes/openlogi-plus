# Fork CI and release setup

This fork reads release credentials directly from GitHub repository secrets.

## Required for release automation

- `RELEASE_BOT_TOKEN`: fine-grained or classic PAT that can create PRs, push tags, create releases, and trigger workflows in this repository. Do not use the default `GITHUB_TOKEN`; tags created by it do not trigger `release.yml`.
- `CARGO_REGISTRY_TOKEN`: crates.io token used by `release-plz` for publishable workspace crates.

## Required for macOS DMG signing, notarization, and updates

- `APPLE_SIGNING_IDENTITY`: Developer ID Application signing identity name.
- `APPLE_CERTIFICATE`: base64-encoded `.p12` Developer ID Application certificate.
- `APPLE_CERTIFICATE_PASSWORD`: password for the `.p12`.
- `APPLE_ID`: Apple ID used for notarization.
- `APPLE_PASSWORD`: app-specific password for notarization.
- `APPLE_TEAM_ID`: Apple Developer Team ID.
- `OPENLOGI_UPDATE_BASE_URL`: public updater origin, for example `https://updates.example.com`.
- `OPENLOGI_UPDATE_MINISIGN_PUBLIC_KEY`: minisign public key embedded into release builds.
- `OPENLOGI_UPDATE_MINISIGN_SECRET_KEY`: base64-encoded minisign secret key file.

## Required for Cloudflare R2 updater publishing

- `CLOUDFLARE_R2_ACCOUNT_ID`
- `CLOUDFLARE_R2_BUCKET`
- `CLOUDFLARE_R2_ACCESS_KEY_ID`
- `CLOUDFLARE_R2_SECRET_ACCESS_KEY`

## Required for Windows signing

- `AZURE_CLIENT_ID`
- `AZURE_TENANT_ID`
- `AZURE_SUBSCRIPTION_ID`
- `AZURE_SIGNING_ENDPOINT`
- `AZURE_SIGNING_ACCOUNT`
- `AZURE_CERT_PROFILE`

The Azure federated credential must allow this subject:

```text
repo:adidoes/openlogi-plus:environment:release
```

## Optional

- `CODEX_ACCESS_TOKEN`, `CHATGPT_USERNAME`, `CODEX_ENDPOINT`: richer generated release notes. Without these, the release notes script falls back to GitHub/changelog text.
- `CROWDIN_PROJECT_ID`, `CROWDIN_PERSONAL_TOKEN`, plus repository variable `CROWDIN_ENABLED=true`: translation sync.
- Repository variables `HOMEBREW_TAP_OWNER` and `HOMEBREW_TAP_REPOSITORY`: dispatch a tap update after publishing. Uses `RELEASE_BOT_TOKEN`.
- Repository variable `OPENLOGI_BUNDLE_ASSETS`: controls whether release builds bundle remote device assets.
