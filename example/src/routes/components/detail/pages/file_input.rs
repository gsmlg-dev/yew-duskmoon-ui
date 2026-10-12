use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::FileInput;

const API: &[ApiRow] = &[
    ApiRow { prop: "class", ty: "Classes", default: "empty", docs: "Additional Core or utility classes." },
    ApiRow { prop: "variant", ty: "Option<String>", default: "None", docs: "Core semantic color modifier: primary, secondary, tertiary, info, success, warning, or error." },
    ApiRow { prop: "accept", ty: "AttrValue", default: "empty", docs: "Accepted native file types." },
    ApiRow { prop: "multiple / disabled / required", ty: "bool", default: "false", docs: "Native selection and control state." },
    ApiRow { prop: "onchange", ty: "Callback<Event>", default: "noop", docs: "Emits the native file selection change event." },
    ApiRow { prop: "id / name / aria_label", ty: "AttrValue", default: "empty", docs: "Native identity and accessible name." },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::FileInput;

html! {
    <label class="form-group"><span class="form-label">{ "Attachments" }</span>
        <FileInput variant={Some("primary".to_owned())} name="attachments" accept="image/*,.pdf" multiple={true} aria_label="Attachments" />
    </label>
}"#.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <label class="form-group"><span class="form-label">{ "Attachments" }</span>
                <FileInput variant={Some("primary".to_owned())} name="attachments" accept="image/*,.pdf" multiple={true} aria_label="Attachments" />
            </label>
    }
}
