use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::RadialProgress;

const API: &[ApiRow] = &[
    ApiRow { prop: "class", ty: "Classes", default: "empty", docs: "Additional Core or utility classes." },
    ApiRow { prop: "variant", ty: "Option<String>", default: "None", docs: "Core semantic color modifier: primary, secondary, tertiary, info, success, warning, or error." },
    ApiRow { prop: "value", ty: "f64", default: "0", docs: "Percentage normalized to 0..100 for CSS, visible text, and ARIA." },
    ApiRow { prop: "children", ty: "Children", default: "empty", docs: "Optional percentage label; default is the normalized percentage." },
    ApiRow { prop: "aria_label", ty: "AttrValue", default: "Progress", docs: "Accessible progressbar name." },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::RadialProgress;

html! {
    <RadialProgress variant={Some("primary".to_owned())} value={68.0} aria_label="Upload progress" />
}"#.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <RadialProgress variant={Some("primary".to_owned())} value={68.0} aria_label="Upload progress" />
    }
}
