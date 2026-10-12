use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Range;

const API: &[ApiRow] = &[
    ApiRow { prop: "class", ty: "Classes", default: "empty", docs: "Additional Core or utility classes." },
    ApiRow { prop: "variant", ty: "Option<String>", default: "None", docs: "Core semantic color modifier: primary, secondary, tertiary, info, success, warning, or error." },
    ApiRow { prop: "min / max / step", ty: "AttrValue", default: "0 / 100 / 1", docs: "Native range bounds and increments." },
    ApiRow { prop: "value", ty: "Option<AttrValue>", default: "None", docs: "Optional controlled value." },
    ApiRow { prop: "oninput / onchange", ty: "Callback", default: "noop", docs: "Native input and change events." },
    ApiRow { prop: "id / name / aria_label", ty: "AttrValue", default: "empty", docs: "Native control identity and accessible name." },
    ApiRow { prop: "disabled / required", ty: "bool", default: "false", docs: "Native control state." },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Range;

html! {
    <label class="form-group"><span class="form-label">{ "Volume" }</span>
        <Range variant={Some("primary".to_owned())} name="volume" aria_label="Volume" />
    </label>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <label class="form-group"><span class="form-label">{ "Volume" }</span>
                <Range variant={Some("primary".to_owned())} name="volume" aria_label="Volume" />
            </label>
    }
}
