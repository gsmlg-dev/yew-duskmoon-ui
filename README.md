# Yew Duskmoon UI

`yew-duskmoon` provides Duskmoon UI components for [Yew](https://yew.rs/) applications.

[![CI](https://github.com/duskmoon-dev/yew-duskmoon-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/duskmoon-dev/yew-duskmoon-ui/actions/workflows/ci.yml)
[![yew-duskmoon crates version](https://badgen.net/crates/v/yew-duskmoon)](https://crates.io/crates/yew-duskmoon)
[![yew-duskmoon crates download](https://badgen.net/crates/d/yew-duskmoon)](https://crates.io/crates/yew-duskmoon)
[![yew-duskmoon crates latest download](https://badgen.net/crates/dl/yew-duskmoon)](https://crates.io/crates/yew-duskmoon)

## Links

- [Documentation and demo](https://duskmoon-dev.github.io/yew-duskmoon-ui/)
- [Crate](https://crates.io/crates/yew-duskmoon)
- [Repository](https://github.com/duskmoon-dev/yew-duskmoon-ui)

## Install

```toml
[dependencies]
yew = { version = "0.23", features = ["csr"] }
yew-duskmoon = "0.9.2"
```

The default `full` feature enables the core components:

- `app_header`
- `button`
- `card`
- `link`
- `typography`

Use explicit features when you want a smaller enabled component surface:

```toml
[dependencies]
yew-duskmoon = { version = "0.9.2", default-features = false, features = ["button", "card"] }
```

## CSS

The Rust components render Duskmoon class names. Include Duskmoon UI styles in the app that consumes this crate.

The demo uses Tailwind CSS 4 with `@duskmoon-dev/core`:

```css
@import "tailwindcss";
@plugin "@duskmoon-dev/core/plugin";
@import "@duskmoon-dev/core";
```

The component markup in this checkout targets the published Core 1.20.4 contracts. The repository demo keeps its local dependency specification, `file:../../duskmoonui/packages/core`, in `example/package.json`; that path resolves to the sibling `../duskmoonui/packages/core` checkout rather than a pinned registry version.

Core supplies component styling and native HTML contracts. Layout primitives use structural modifiers; they do not all have color variants. The new colorable primitives support `primary`, `secondary`, `tertiary`, `info`, `success`, `warning`, and `error`; use only modifiers supported by the component's Core CSS.

If your CSS build removes unused classes, make sure it scans your Yew source files and the component sources you use.

## Usage

```rust
use yew::prelude::*;
use yew_duskmoon::{Button, ButtonAppearance, Card, Color};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Card title={html! { "Quick start" }}>
            <Button
                appearance={Some(ButtonAppearance::Filled)}
                color={Some(Color::Primary)}
            >
                { "Save" }
            </Button>
        </Card>
    }
}
```

## Data-management primitives

`Color` provides the shared core palette (`Primary`, `Secondary`, `Tertiary`, `Accent`, `Neutral`, `Base`, `Info`, `Success`, `Warning`, and `Error`) for components whose CSS contracts support it. Legacy `variant` strings and custom classes remain available as escape hatches.

Typed badge and button modifiers cover their common core-supported forms:

```rust
use yew::prelude::*;
use yew_duskmoon::{
    Badge, BadgeAppearance, BadgeSize, Button, ButtonAppearance, ButtonSize,
    Color, NativeButtonType,
};

html! {
    <>
        <Badge
            appearance={BadgeAppearance::Tonal}
            size={BadgeSize::Small}
            color={Some(Color::Neutral)}
            title="Internal account"
        >
            { "Internal" }
        </Badge>

        <Button
            native_type={NativeButtonType::Submit}
            appearance={Some(ButtonAppearance::Outlined)}
            size={ButtonSize::Small}
            color={Some(Color::Primary)}
        >
            { "Save" }
        </Button>
    </>
}
```

`BadgeAppearance` supports `Filled`, `Tonal`, and `Outlined`; `BadgeSize` supports `Small`, `Medium`, and `Large`. Badges render as `<span>` elements, accept optional `role`, `aria_label`, and `title` attributes, and do not receive a status role by default. If both `variant` and `color` are set, `variant` wins while typed appearance and size still apply.

`ButtonAppearance` supports `Filled`, `Outlined`, `Tonal`, and `Text`; `ButtonSize` supports `Small`, `Medium`, and `Large`. Native buttons default to `NativeButtonType::Button`, so they do not submit a surrounding form unless `Submit` is selected. Buttons also accept `aria_label`, `aria_describedby`, `aria_pressed`, `aria_expanded`, `title`, and `tooltip_id`. Disabled and loading controls suppress their callbacks, and loading controls expose `aria-busy="true"`.

`IconButton` renders a native `<button>` and requires a `label`, which becomes its `aria-label`. It defaults to `NativeButtonType::Button`, `ButtonAppearance::Text`, and `ButtonSize::Medium`; use `appearance`, the shared `Color` enum, and `size` for supported visual combinations, with `class` as an escape hatch. It accepts `native_type`, suppresses callbacks while `disabled` or `loading`, exposes loading state through `aria-busy`, and supports `title` as a fallback plus `tooltip_id` for a native tooltip relationship.

### Semantic tables and native tooltips

`Table` accepts semantic table children directly, optional `aria_label` and `aria_describedby` attributes, and the core responsive wrapper. `TableDensity` selects `Default`, `Compact`, or `Comfortable`; `TableBorders` selects `Default`, `Bordered`, or `Borderless`. The `hoverable`, `striped`, `sticky_header`, `selectable`, and `surface` booleans map to their matching core modifiers. Use `sticky_header` inside a scroll container where the sticky behavior has a meaningful boundary.

```rust
use yew::prelude::*;
use yew_duskmoon::{
    Badge, BadgeAppearance, BadgeSize, ButtonSize, Color, IconButton, Table,
    TableBorders, TableDensity, Tooltip, TooltipContent, TooltipPlacement,
    TooltipTone,
};

html! {
    <Table
        responsive={true}
        hoverable={true}
        density={TableDensity::Compact}
        borders={TableBorders::Bordered}
    >
        <caption>{ "Account access" }</caption>
        <thead>
            <tr>
                <th scope="col">{ "Account" }</th>
                <th scope="col">{ "Status" }</th>
                <th scope="col">{ "Actions" }</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>{ "Example Co." }</td>
                <td>
                    <Badge
                        appearance={BadgeAppearance::Tonal}
                        size={BadgeSize::Small}
                        color={Some(Color::Success)}
                    >
                        { "Active" }
                    </Badge>
                </td>
                <td>
                    <IconButton
                        label="Edit Example Co."
                        size={ButtonSize::Small}
                        color={Some(Color::Primary)}
                        tooltip_id="edit-example-tooltip"
                        title="Edit Example Co."
                    >
                        <span aria-hidden="true">{ "✎" }</span>
                    </IconButton>
                    <Tooltip
                        id="edit-example-tooltip"
                        placement={TooltipPlacement::Bottom}
                        content={TooltipContent::Plain}
                        tone={TooltipTone::Dark}
                    >
                        { "Edit account" }
                    </Tooltip>
                </td>
            </tr>
        </tbody>
    </Table>
}
```

Each `Tooltip` is a sibling surface with a required stable `id`. Setting a `Button` or `IconButton` `tooltip_id` to that same value supplies `interestfor`, `aria-describedby`, and the matching CSS anchor without cloning the trigger. Keep visible text or an independent `aria_label`/`label` as the trigger's accessible name; `title` is a useful fallback. Placements are `Top`, `Bottom`, `Left`, and `Right`; content forms are `Plain`, `Multiline`, and `Rich`; tones are `Dark` and `Light`, with `Color` available for palette variants. A legacy `variant` wins over `color`, which wins over `tone`. Native display relies on the Popover API, `interestfor`, and CSS Anchor Positioning, so tooltip behavior is progressive enhancement on browsers without those features.

Native tooltip overflow and sticky table-header fixes are available in `@duskmoon-dev/core` 1.18.6 and later. This Yew wrapper emits those contracts without local CSS overrides.

### Native popovers and dialogs

`Popover` requires a stable `id` and renders a command trigger next to a native `popover` surface. The default `PopoverMode::Auto` provides light-dismiss; use `Manual` with an explicit `hide-popover` command when dismissal must be controlled. The component also emits matching CSS anchor styles, so no Yew visibility state or `popover-show` class is needed.

Use `trigger_onclick` for an application action and set `trigger_command_enabled={false}` to disable click commands. Set `trigger_interest={true}` for native hover/focus descriptions in browsers that support `interestfor`. The visible `trigger_label` and optional `trigger_aria_label` can differ; `trigger_aria_describedby`, `trigger_aria_haspopup`, and `trigger_aria_expanded` are optional and caller-managed. Leave popup and expansion ARIA unset for descriptive content.

`Dialog` renders a native `<dialog>` and requires an `id`. `Button` and `IconButton` accept `command` and `command_for`, which map to the HTML `command` and `commandfor` attributes:

```rust
use yew::prelude::*;
use yew_duskmoon::{Button, Dialog, Popover};

html! {
    <>
        <Popover id="account-menu" class="popover-bottom">
            <div class="popover-body">{ "Account options" }</div>
        </Popover>

        <Button command="show-modal" command_for="confirm-delete">{ "Delete" }</Button>
        <Dialog id="confirm-delete" aria_label="Confirm deletion">
            <div class="dialog-box">
                <div class="dialog-body">{ "Delete this item?" }</div>
                <div class="dialog-footer">
                    <Button command="close" command_for="confirm-delete">{ "Cancel" }</Button>
                </div>
            </div>
        </Dialog>
    </>
}
```

`Dropdown`, `Fab`, and `Megamenu` require `id` and `trigger_label`; they generate native `popovertarget` invokers with `popover="auto"` surfaces. `DropdownPlacement` selects `BlockStart`, `BlockEnd`, `InlineStart`, or `InlineEnd`. Keep each target id unique. `Drawer` and `BottomSheet` now render modal dialogs by default: give them an `id`, open them with `show-modal`, and close them with `close`. With `modal={false}`, they render auto popovers and use `show-popover` / `hide-popover` commands. Preserve the matching `.drawer-*` or `.bottomsheet-*` child structure.

Core 1.20.4 can place a Megamenu panel outside a narrow viewport in a tall, scrolling document, including at the 768px desktop breakpoint ([upstream issue #75](https://github.com/duskmoon-dev/duskmoonui/issues/75)). The demo composes `.megamenu-desktop` disclosure with `.megamenu-mobile` in-flow `Menu` links so mobile destinations remain reachable. Narrow desktop disclosure can still overflow until the upstream CSS is fixed; browser-native focus and dismissal work normally.

Toast notifications use a native manual-popover container rather than overlay visibility classes:

```rust
use yew::prelude::*;
use yew_duskmoon::{Button, Toast};

html! {
    <>
        <Button command="show-popover" command_for="saved-toast">{ "Show notification" }</Button>
        <div id="saved-toast" class="toast-container toast-container-bottom-end" popover="manual">
            <Toast variant={Some("success".to_owned())}>
                <span class="toast-content" role="status"><span class="toast-message">{ "Saved" }</span></span>
                <button type="button" class="toast-close" command="hide-popover" commandfor="saved-toast" aria-label="Dismiss notification">{ "Dismiss" }</button>
            </Toast>
        </div>
    </>
}
```

`Modal` remains as a legacy name but now implements the new native dialog contract. Migrating existing `Modal` calls is required; prefer `Dialog` in new code.

### Native form controls and disclosure

`Input` renders a native input by default and exposes `input_type`, `id`, `name`, `value`, `placeholder`, `aria_label`, `required`, `disabled`, `readonly`, `oninput`, and `onchange`. Text children become its value. `Select` renders a native select with `option` or `optgroup` children; legacy text-only children become one option. Its native props include `id`, `name`, `value`, `aria_label`, `required`, `disabled`, `multiple`, and `onchange`.

`Checkbox`, `Radio`, and `Switch` render native inputs when children are omitted. `ToggleSwitch` is the new native checkbox wrapper for Core's `.toggle` contract; the existing `Toggle` remains a group of pressed-state buttons. Omit `checked: Option<bool>` to retain browser-managed toggling, or supply it and update application state through `onchange`. Use `name` and `value` for submitted values, and provide a visible label or `aria_label`.

```rust
use yew::prelude::*;
use yew_duskmoon::{Checkbox, Form, Input, Select};

html! {
    <Form onsubmit={Callback::from(|event: SubmitEvent| event.prevent_default())}>
        <label class="form-group">
            <span class="form-label">{ "Email" }</span>
            <Input name="email" input_type="email" required={true} placeholder="you@example.com" />
        </label>
        <label class="form-group">
            <span class="form-label">{ "Environment" }</span>
            <Select name="environment">
                <option value="production">{ "Production" }</option>
                <option value="staging">{ "Staging" }</option>
            </Select>
        </label>
        <label class="checkbox-label">
            <Checkbox name="updates" />
            <span>{ "Send release notes" }</span>
        </label>
    </Form>
}
```

`Form` now renders `<form class="form-group">`; its `onsubmit` callback receives the native `SubmitEvent`. `FormGroup` retains the label/control/helper composition. `Validator` groups native `.validator` controls with `.validator-hint` and `.validator-error` text; `immediate={true}` opts into `.validate` styling before interaction. Applications own validation messages and announcements. `Range` and `FileInput` render native range/file inputs with their corresponding bounds, file constraints, names, labels, and event callbacks. `FilterGroup` composes direct `label.chip` children containing native `.filter-group-input` radio or checkbox controls.

Existing child composition remains available: structured or custom component children keep `Input` in composition mode; nested native select markup or `composition={true}` keeps `Select` in composition mode. Custom option components are forwarded inside the native select by default; use `composition={true}` when a custom component renders the complete select. Children on `Checkbox`, `Radio`, or `Switch` create a group wrapper. In composition mode, supply the native child classes, attributes, labels, and handlers yourself. A supplied `Select.value` sets the native selection on mount and render; omitting it preserves browser-managed selection. `Textarea` remains a native textarea; its text children provide the initial value and `auto_size` selects Core's auto-resize class.

`Progress` renders native `<progress>` when children are empty. Set `value` and `max` for determinate progress, or omit `value` for indeterminate progress. Existing custom `.progress-bar` children retain custom composition. `RadialProgress` normalizes its percentage for both the CSS value and ARIA; `Loading` provides spinner, dots, or bars. `Countdown` presents ordinary `.countdown-value` text; the application owns timer updates.

`Collapse` renders native `<details>` with a `<summary class="collapse-trigger">` child. `open={true}` supplies the initial expanded state; legacy `collapse-open` and `show` classes also initialize it as open. Browser disclosure state controls later keyboard and pointer toggling.

```rust
use yew::prelude::*;
use yew_duskmoon::Collapse;

html! {
    <Collapse open={true} class="collapse-bordered">
        <summary class="collapse-trigger">{ "Deployment details" }</summary>
        <div class="collapse-content"><div>{ "Build and verification results." }</div></div>
    </Collapse>
}
```

### Complete native OTP and PIN values

With no children, `OtpInput` renders one complete native input over decorative cells, preserving paste, browser editing, and `autocomplete="one-time-code"`. Its `length` defaults to six and is clamped to Core's supported one to eight cells. Use `id`, `name`, `aria_label`, optional `value`, `required`, `disabled`, `oninput`, and `onchange` on the generated input.

`PinInput` remains a public compatibility name and uses the current OTP contract. It defaults to four digits, a password input, and `autocomplete="off"`. Core 1.20.4 does not ship `.pin-input*` styles: migrate supplied classes and child markup to `.otp-input*`, `.otp-code`, or the legacy `.otp-input-field` contract. The removed PIN circle styling has no dedicated replacement.

```rust
use yew::prelude::*;
use yew_duskmoon::{OtpInput, PinInput};

html! {
    <>
        <OtpInput name="verification_code" length={6} aria_label="Verification code" required={true} />
        <PinInput name="security_pin" length={4} aria_label="Security PIN" required={true} />
    </>
}
```

Explicit children preserve manual OTP/PIN composition; the caller owns the native input attributes, value, events, and matching cell markup in that mode.

## Migration notes

- `Input`, `Select`, `Checkbox`, `Radio`, `Switch`, and `Progress` now support native direct rendering. Replace control-shaped `div` markup and synthetic demo state classes with native values, checked state, labels, and callbacks; optional child composition remains available as described above.
- `Form` now renders a native form; handle `onsubmit` when the application needs to prevent default navigation. `Collapse` now requires native summary/content children for browser-managed disclosure.
- `OtpInput` defaults to one complete native code; `PinInput` remains compatible at the Rust component level but its rendered classes use OTP CSS. Replace all removed `.pin-input*` classes in composed content.
- `Drawer` and `BottomSheet` now use native dialog or popover state. Replace open-state CSS classes with native invokers targeting a stable id. `NestedMenu` now renders a list and expects native `li`/`details`/`summary` composition.

- `Badge` now renders a `<span>` instead of a `<div>`. Update selectors or layout assumptions that depended on the old root.
- `Button` now defaults to native `type="button"`; set `native_type={NativeButtonType::Submit}` for form submission. `ButtonType::Link` now uses the supported `btn-text` class instead of the old unsupported `btn-link`. A disabled or loading link no longer emits `disabled` or `href`, is removed from normal keyboard activation, and exposes `aria-disabled`.
- `Table` now renders a semantic `<table>`. Replace the old `<div>` row markup with `<caption>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, and `<td>` as appropriate. With `responsive={true}`, custom `class` stays on the table and `wrapper_class` customizes the `.table-responsive` wrapper.
- `Tooltip` replaces the obsolete wrapper/`.tooltip-content`/`.tooltip-open` structure with a `popover="hint"` surface. Give it a stable `id` and associate a separate trigger through `tooltip_id`.
- `Popover` no longer accepts `PopoverTrigger` or manages open state. Give it a stable `id`; the generated button uses `command="toggle-popover"`, and browser state is represented by `:popover-open`.
- `Dialog` and the legacy `Modal` name now require an `id` and render a native `<dialog>`. This is a breaking markup migration: replace `.modal-open`, old `.modal-*` children, conditional overlay wrappers, and click-state handlers with the `.dialog-*` structure and `show-modal` / `close` commands.

## Asynchronous syntax highlighting

`CodeEditor` and `DmMarkdown` request Tree-sitter highlighting in a Web Worker. The editor reuses its syntax tree for incremental parsing; Markdown highlights fenced code and visible YAML front matter. Both show plain source while requests are pending or unavailable and discard outdated results. Pass `syntax_service` to share a configured `SyntaxService`. The synchronous Markdown HTML rendering functions retain their existing highlighting behavior.

All 33 code modes in `CodeLanguage` have bundled grammars and highlight queries, with Go and TSX also available through the syntax service. Markdown, HTML, and PHP use upstream injection queries to highlight embedded languages. Plain text skips parsing; grammars load on demand from the application's own assets.

Browser highlighting requires exporting the bundled assets before the application's asset-copy step. For Trunk, run the bundled exporter in a pre-build hook so the directory exists before `copy-dir` validation; the demo does this automatically. Serve the exported directory at `tree-sitter/` relative to `document.baseURI`, or set `assets_url` in `SyntaxConfig` to configure another location. See the [Code Engine asset setup](packages/code-engine/README.md) for the export command and required Trunk copy configuration.

## Bounded Markdown rendering

`DmMarkdown` and the direct HTML renderers support Markdown footnotes. For export jobs that need explicit limits, use `render_markdown_to_html_with_limits` with the same `DmMarkdownOptions` used by the ordinary renderer:

```rust
use yew_duskmoon::{
    render_markdown_to_html_with_limits, DmMarkdownLimits, DmMarkdownOptions,
};

let html = render_markdown_to_html_with_limits(
    "A reference[^note].\n\n[^note]: Details.",
    DmMarkdownOptions::default(),
    DmMarkdownLimits {
        max_input_bytes: 1024 * 1024,
        max_output_bytes: 8 * 1024 * 1024,
        max_mermaid_bytes: 16 * 1024,
    },
)?;
```

`max_input_bytes` includes the Markdown source and URL/custom-element configuration. `max_output_bytes` caps generated HTML while it is written, including syntax highlighting, front matter, color chips, and Mermaid diagrams. `max_mermaid_bytes` caps each diagram's source before parsing and layout. A limit violation returns `DmMarkdownRenderError` without a partial HTML result. These byte limits do not specify total process memory or a wall-clock timeout. The existing rendering APIs keep their unlimited behavior.

## Components

### Additional Core 1.20.4 wrappers

This checkout adds 26 exported wrappers:

| Surface | Exports |
| --- | --- |
| Actions and navigation | `Dropdown`, `Fab`, `Swap`, `Megamenu` |
| Native input composition | `Validator`, `Range`, `FileInput`, `FilterGroup`, `ToggleSwitch` |
| Activity and progress | `Loading`, `RadialProgress`, `Countdown` |
| Data display | `Stat`, `Kbd`, `Carousel`, `Diff`, `Indicator` |
| Layout | `ConsolePage`, `Footer`, `Hero`, `HomePage`, `Join`, `Mask`, `SidebarLayout`, `SignPage`, `Stack` |

These wrappers compose Core's documented slots: for example, `.stats` contains `Stat` items, `Carousel` contains `.carousel-item` children, `Join` contains `.join-item` controls, and `Indicator` positions `.indicator-item` children using slot classes such as `indicator-top indicator-end`. `Indicator` accepts `class` and `children`; its positions belong on the item. `Diff.position` controls the percentage revealed through `--diff-position`. `Carousel` provides a named, keyboard-focusable native scroll region. `Swap` is a controlled toggle button: supply `label`, non-interactive `on`/`off` indicators, `pressed`, and `on_change` to update application state.

`Chat` semantic variants style native `.chat-bubble` descendants; alignment uses `chat-start` or `chat-end` on the root. `Link` uses the Core `.link` contract. CodeBlock and MarkdownBody use their structural slots rather than invented color modifiers. ThemeController styles native radios and adjacent labels; applications own theme changes. Timeline colors belong on `.timeline-item-*` or `.timeline-marker-*` slots.


The crate includes components for actions, data display, data entry, feedback, layout, navigation, and surfaces.

Common exports include:

`Accordion`, `Alert`, `Appbar`, `Autocomplete`, `Avatar`, `Badge`, `BottomNavigation`, `Breadcrumbs`, `Button`, `Card`, `Checkbox`, `Chip`, `CodeBlock`, `CodeEditor`, `DatePicker`, `Dialog`, `DmMarkdown`, `Drawer`, `FileUpload`, `Form`, `Grid`, `IconButton`, `Input`, `Link`, `List`, `MarkdownBody`, `Menu`, `Modal`, `Pagination`, `Progress`, `Radio`, `Select`, `Switch`, `Table`, `Tabs`, `Textarea`, `ThemeController`, `Toast`, `Tooltip`, `TreeSelect`, and `Typography`.

See the [demo](https://duskmoon-dev.github.io/yew-duskmoon-ui/) for the full catalog.

## Development

Install the Rust WebAssembly target and Trunk:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
```

The demo requires the sibling Core checkout at `<repo-root>/../duskmoonui`. Prepare its generated package assets first, using Core 1.20.4 for this compatibility target:

```sh
cd ../duskmoonui
bun install --frozen-lockfile
bun run build:core
cd ../yew-duskmoon-ui
```

Run the demo:

```sh
cd example
bun install
bun run build:css
trunk serve
```

Run crate checks from the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```
