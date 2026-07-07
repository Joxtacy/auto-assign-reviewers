# Releasing

Releases are driven by **publishing a GitHub Release**. That triggers
`.github/workflows/release.yaml`, which builds the Docker image and pushes it to
`ghcr.io/Joxtacy/auto-assign-reviewers`.

A pre-release is identical to a release except for the `--prerelease` flag on the
final step. Do the prep on `main`.

## 1. Prep (on `main`)

Pick the next version, e.g. `0.2.0`. Then, **on a release-prep commit** (so the
tag captures the exact-version image pin — see below):

- [ ] Pin **`action.yaml`** `runs.image` to the version being released:
      `docker://ghcr.io/joxtacy/auto-assign-reviewers:0.2.0` (no `v` prefix — the
      release workflow's metadata-action strips it). This is what makes
      `@v0.2.0` deterministically run the `0.2.0` image. `main` itself stays on
      `:latest`; only the tagged release commit carries the exact pin. **Easy to
      miss — if you skip it, the release runs the previous image.**
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
- [ ] The tagged `action.yaml` `runs.image` points at the new version's image
      (not the previous one).

## 4. Publish to the GitHub Marketplace (web UI only)

The Marketplace toggle is **not** exposed by `gh` or the REST API — it can only
be set through the release page on github.com.

- [ ] Open `https://github.com/Joxtacy/auto-assign-reviewers/releases/edit/vX.Y.Z`
- [ ] Tick **"Publish this Action to the GitHub Marketplace"**, confirm the
      category, and **Update release**.

Requires `action.yaml` at the repo root with a `branding:` block (already
present). Skip this for pre-releases.

## Gotchas

- **Tag shape controls `latest`, not GitHub's "pre-release" checkbox.**
  `docker/metadata-action` only parses the git tag string — it never sees the
  release's pre-release flag. So what matters is whether the tag is a prerelease
  semver:
  - Stable tag `v0.2.0` → emits `0.2.0`, `0.2`, `0`, **and `latest`**. Ticking
    GitHub's "pre-release" box does **not** stop `latest` from moving.
  - Prerelease tag `v0.2.0-rc.1` → emits **only** `0.2.0-rc.1` (+ `sha-…`). No
    `0.2`/`0`, no `latest`.
  - **To pre-release without touching `latest`, the tag must carry a prerelease
    suffix** (`-rc.1`, `-beta.1`, …).
- **`type=raw,value=latest,enable={{is_default_branch}}` does not fire on
  releases.** A `release: published` event has a tag ref, so `is_default_branch`
  is false. That raw `latest` only applies on the `workflow_dispatch` test path
  (built from `main`) — which is how the current `latest` tag was set.
- **`type=semver,pattern={{version}}` strips the leading `v`**, so tag `v0.2.0`
  becomes image tag `0.2.0`.
