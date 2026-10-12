use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::ToggleSwitch;

const API: &[ApiRow] = &[
    ApiRow { prop: "class", ty: "Classes", default: "empty", docs: "Additional Core or utility classes." },
    ApiRow { prop: "variant", ty: "Option<String>", default: "None", docs: "Core semantic color modifier: primary, secondary, tertiary, info, success, warning, or error." },
    ApiRow { prop: "checked", ty: "Option<bool>", default: "None", docs: "Optional controlled checked state; omission preserves native toggling." },
    ApiRow { prop: "value", ty: "AttrValue", default: "on", docs: "Native submitted checkbox value." },
    ApiRow { prop: "onchange", ty: "Callback<Event>", default: "noop", docs: "Emits native change events." },
    ApiRow { prop: "id / name / aria_label", ty: "AttrValue", default: "empty", docs: "Native identity and accessible name." },
    ApiRow { prop: "disabled / required", ty: "bool", default: "false", docs: "Native control state." },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::ToggleSwitch;

html! {
    <label class="switch-label">
        <ToggleSwitch variant={Some("primary".to_owned())} name="notifications" />
        <span>{ "Enable notifications" }</span>
    </label>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <label class="switch-label">
                <ToggleSwitch variant={Some("primary".to_owned())} name="notifications" />
                <span>{ "Enable notifications" }</span>
            </label>
    }
}
