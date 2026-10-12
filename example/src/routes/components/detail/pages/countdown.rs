use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Countdown;

const API: &[ApiRow] = &[
    ApiRow { prop: "class", ty: "Classes", default: "empty", docs: "Additional Core or utility classes." },
    ApiRow { prop: "variant", ty: "Option<String>", default: "None", docs: "Core semantic color modifier: primary, secondary, tertiary, info, success, warning, or error." },
    ApiRow { prop: "children", ty: "Children", default: "empty", docs: "Ordinary countdown-value text; the application updates timer state." },
    ApiRow { prop: "aria_label", ty: "Option<AttrValue>", default: "None", docs: "Optional accessible timer name." },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Countdown;

html! {
    <Countdown variant={Some("primary".to_owned())} aria_label="Time remaining">
        <span><span class="countdown-value">{ "02" }</span>{ " minutes" }</span>
        <span><span class="countdown-value">{ "30" }</span>{ " seconds" }</span>
    </Countdown>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Countdown variant={Some("primary".to_owned())} aria_label="Time remaining">
                <span><span class="countdown-value">{ "02" }</span>{ " minutes" }</span>
                <span><span class="countdown-value">{ "30" }</span>{ " seconds" }</span>
            </Countdown>
    }
}
