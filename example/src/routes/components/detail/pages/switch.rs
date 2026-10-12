use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Switch;

const SWITCH_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the Switch wrapper.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Composition children; omission renders a native input.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a switch color class such as switch-primary.",
    },
    ApiRow {
        prop: "id / name / value",
        ty: "AttrValue",
        default: "empty / empty / on",
        docs: "Native control identity and submitted value in direct mode.",
    },
    ApiRow {
        prop: "checked",
        ty: "Option<bool>",
        default: "None",
        docs: "Controlled state when set; omission retains native browser toggling.",
    },
    ApiRow {
        prop: "disabled / required",
        ty: "bool",
        default: "false",
        docs: "Native control state in direct mode.",
    },
    ApiRow {
        prop: "onchange",
        ty: "Callback<Event>",
        default: "noop",
        docs: "Native change events in direct mode.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, SWITCH_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Switch;\n\nhtml! {\n    <label class=\"switch-label\">\n        <Switch variant={Some(\"success\".to_owned())} class=\"demo-switch\" />\n        <span>{ \"Enable notifications\" }</span>\n    </label>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="switch-group">
            <label class="switch-label">
                <Switch variant={primary_variant()} class="demo-switch" />
                <span>{ "Auto-save drafts" }</span>
            </label>
            <label class="switch-label">
                <Switch class="demo-switch" />
                <span>{ "Send weekly summary" }</span>
            </label>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <label class="switch-label">
            <Switch variant={variant(color)} class="demo-switch" />
            <span>{ color.label }</span>
        </label>
    }
}
