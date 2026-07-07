# Releasing

Releases are driven by **publishing a GitHub Release**. That triggers
`.github/workflows/release.yaml`, which builds the Docker image and pushes it to
`ghcr.io/Joxtacy/auto-assign-reviewers`.

A pre-release is identical to a release except for the `--prerelease` flag on the
final step. Do the prep on `main`.

## 1. Prep (on `main`)

Pick the next version, e.g. `0.2.0`. Then:

- [ ] Bump `version` in **`Cargo.toml`** (this is easy to forget — it does not
      auto-update from the tag).
- [ ] Update **`CHANGELOG.md`**: move `[Unreleased]` items under a new
      `## [0.2.0] - YYYY-MM-DD` heading, and update the compare links at the
      bottom of the file.
- [ ] Bump the `@vX.Y.Z` action refs consumers pin to — **`README.md`** (3
      places) and **`example-workflow.yaml`**.
- [ ] Commit and push to `main`.

## 2. Cut the release

`gh release create` creates the git tag at `--target`, so no separate `git tag`
step is needed. Publishing the release is what starts the image build/push.

Stable:

```bash
gh release create v0.2.0 --title "v0.2.0" --notes-from-tag --target main
```

Pre-release:

```bash
gh release create v0.2.0-rc.1 --prerelease --title "v0.2.0-rc.1" --target main
```

## 3. Verify

- [ ] Workflow succeeded: `gh run watch` (or the Actions tab).
- [ ] Image tags landed on GHCR. For `v0.2.0` you get `0.2.0`, `0.2`, `0`,
      `latest`, and a `sha-…` tag.

## Gotchas

- **`latest` moves on any release cut from `main`, including pre-releases** — the
  workflow keys `latest` off `enable={{is_default_branch}}`, not the release
  type. A pre-release from a *plain* tag (like `v0.1.1`) will overwrite `latest`.
- **Pre-release semver tags** (e.g. `v0.2.0-rc.1`) are treated as pre-release by
  `docker/metadata-action`: it emits only the full `0.2.0-rc.1` tag, not the
  short `0.2`/`0` tags, and does not touch `latest`.
