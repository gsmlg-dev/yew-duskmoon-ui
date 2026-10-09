---
name: cmd-sync-duskmoonui
description: Sync Yew components and the Trunk demo with the latest published DuskMoonUI CSS and native HTML contracts, adapting dependency updates to this repository's local Core source link.
disable-model-invocation: true
---

# Sync DuskMoonUI Dependencies and Yew Components

## Input and scope

Accept optional `--only=core` or `--only=css-art`. By default, sync the upstream packages already consumed by this repository.

Currently only `@duskmoon-dev/core` is used, through `example/package.json` with `file:../../duskmoonui/packages/core`. There is no CSS Art dependency or art component subsystem. For `--only=css-art`, report that it is not integrated and stop; adding that integration is a separate task.

Core supplies CSS and HTML contracts; `yew-duskmoon` supplies Rust components that render matching markup. A successful sync includes an API audit even when no dependency version changes.

## Steps

### 1. Establish the current integration

Read applicable `AGENTS.md` instructions and record the current branch and staged/unstaged changes. Work in the current checkout and preserve unrelated work, including changes in the sibling `../duskmoonui` repository.

Inspect these integration points:

| Surface | Files |
|---------|-------|
| Core dependency and lockfile | `example/package.json`, `example/bun.lock` |
| CSS imports, plugin and source scanning | `example/static/app.css`, `example/static/index.html` |
| Yew components and exports | `packages/duskmoon/src/components/`, `packages/duskmoon/src/components/mod.rs`, `packages/duskmoon/src/lib.rs` |
| Legacy modules and feature gates | `packages/duskmoon/src/`, `packages/duskmoon/Cargo.toml` |
| Component inventory | `example/src/routes/components/catalog.rs` |
| Component demos and registration | `example/src/routes/components/detail/pages/`, its `mod.rs` |

Resolve the latest published version from npm:

```sh
npm view @duskmoon-dev/core version
```

Record the dependency specification separately from the linked package's version, Git revision and dirty state. The local path in `example/bun.lock` does not establish a previous numeric Core version. If the sibling is missing, use a temporary upstream reference for the audit and report the missing prerequisite for the normal demo build.

### 2. Inspect upstream public changes

Use a temporary clone or fetched reference of `https://github.com/duskmoon-dev/duskmoonui.git` to inspect the published target tag. Fetch sufficient history and verify tag names before comparing a known previous version with the target. Do not switch branches, reset or pull the sibling checkout as part of this sync.

Compare `packages/core/package.json` exports, `packages/core/src/index.css`, `packages/core/src/components/`, `packages/core/src/themes/`, `packages/core/src/tailwind-plugin.ts` and relevant public types and documentation. Public CSS-only entrypoints count even when absent from TypeScript barrels.

Inspect the target npm package in a temporary directory with `npm pack --ignore-scripts @duskmoon-dev/core@<target-version>` to confirm the shipped exports and CSS. If a previous version cannot be established, audit the current wrappers against the target inventory and state that the historical diff baseline is unknown.

Map upstream entries to Rust components and the demo catalog, including intentional aliases and local-only components. Check classes, variants, DOM structure, slots, native attributes, accessibility and interaction behavior. Ignore internal changes that do not affect consumers.

Do not stop at matching version numbers. Report "already up to date" only after the public contract audit finds no required changes.

### 3. Update dependencies without changing their source mode

- Preserve `file:../../duskmoonui/packages/core`. Do not replace it with a registry version or claim that a path dependency was version-bumped.
- If registry specifications are present, update every relevant manifest occurrence, preserving `^`, `~` or exact ranges, and regenerate the corresponding lockfile with Bun.
- For linked Core, prepare its build output using the upstream build scripts and read its own instructions before doing so. Generated assets are not source changes to commit.
- When the linked checkout differs from the published target, validate the target in a disposable copy of this repository: point only that copy's dependency at the extracted npm package and reinstall before building its CSS and demo. Leave the checked-in path and sibling source untouched. Distinguish results for linked source from results for the published target.

Do not add unused CSS Art dependencies, bump Cargo crate versions or edit generated `example/static/tailwind.css`.

### 4. Sync Yew components and demos

Apply the audited changes using existing Rust/Yew conventions:

| Upstream change | Action |
|-----------------|--------|
| New public component | Add the corresponding Yew wrapper, exports, catalog entry and demo; explain entries that need no dedicated Rust wrapper |
| Removed or renamed contract | Migrate rendered markup and demos; preserve public Rust aliases or deprecate them rather than silently deleting exported APIs |
| Changed CSS class, variant or HTML structure | Update component rendering, property mappings and all affected demo usage |
| Changed native behavior | Align attributes, invokers, anchor relationships, callbacks and accessibility with the upstream browser contract |
| New optional capability | Add it when needed to expose the updated component contract; avoid unrelated features |

Preserve existing feature gates and public aliases. Keep `packages/code-engine/` unchanged unless an identified sync requirement affects it. Update relevant consumer documentation when the required markup or API changes.

For overlays, retain native dialog/popover behavior: browser-managed modality, focus and Escape handling; `popover="hint"` and `interestfor` where required; matching invoker targets and CSS anchors. Verify the latest upstream contract rather than recreating it with CSS visibility toggles.

Audit existing upstream workarounds before removing them. Route new upstream defects through the applicable internal-request policy: create a typed issue with the `internal request` label, record a language-appropriate `TODO(upstream)` at the callsite, and stop the affected task for blockers. Mark permitted non-blocking workarounds with `WORKAROUND(upstream)` and the issue reference.

### 5. Verify the library and demo separately

Add focused regression tests beside components for changed behavior and run targeted tests while iterating, such as `cargo test -p yew-duskmoon tooltip`. Fix failures caused by this sync; report unrelated failures without expanding scope. Honor any user-specified PRD test boundaries.

For a complete sync, run the repository gates from the root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace --all-features
cargo test --workspace --all-features
cargo fmt --manifest-path example/Cargo.toml -- --check
```

For the existing linked-source integration, prepare Core in the sibling repository:

```sh
bun install --frozen-lockfile
bun run build:core
```

Then run in this repository's `example/`:

```sh
bun install
bun run build:css
trunk build --release --public-url /yew-duskmoon-ui/
```

After intentionally updating the lockfile, check reproducibility with `bun install --frozen-lockfile`. `example/` is excluded from the Cargo workspace; workspace success does not verify its CSS or Wasm build. If the linked source differs from the target, also run the CSS and Trunk gates in the disposable target-package copy described above.

Use `trunk serve` in `example/` for browser checks of affected demos at desktop and mobile widths. When the linked source differs from the target, serve the disposable published-package copy for these checks and report any linked-source browser results separately. Verify styling, overflow, interaction and console errors. For changed overlays, exercise anchoring, light dismiss, Escape, focus and modal inertness. Capture screenshots for visible demo changes. State any unavailable browser or target-package verification explicitly.

### 6. Report and stop

Run `git diff --check` and inspect the final diff. Report the published target, actual linked version/revision, manifest or lockfile changes, components added or migrated, preserved compatibility aliases, validation outcomes and remaining upstream blockers.

Do not commit generated `target/`, `dist/`, `node_modules/` or `example/static/tailwind.css`. Commit or push only when requested, staging only sync files. Release is a separate maintainer action through `.github/workflows/release.yml`; do not manually bump or publish crates as part of this skill.
