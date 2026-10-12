use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Loading;

const API: &[ApiRow] = &[
    ApiRow { prop: "class", ty: "Classes", default: "empty", docs: "Additional Core or utility classes." },
    ApiRow { prop: "variant", ty: "Option<String>", default: "None", docs: "Core semantic color modifier: primary, secondary, tertiary, info, success, warning, or error." },
    ApiRow { prop: "indicator", ty: "AttrValue", default: "spinner", docs: "Core indicator: spinner, dots, or bars." },
    ApiRow { prop: "size", ty: "Option<String>", default: "None", docs: "Core size: xs, sm, md, lg, or xl." },
    ApiRow { prop: "aria_label", ty: "AttrValue", default: "Loading", docs: "Accessible activity description." },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Loading;

html! {
    <div class="flex items-center gap-6">
        <Loading variant={Some("primary".to_owned())} aria_label="Loading preview" />
        <Loading indicator="dots" variant={Some("secondary".to_owned())} aria_label="Loading messages" />
        <Loading indicator="bars" variant={Some("tertiary".to_owned())} aria_label="Loading assets" />
    </div>
}"#.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="flex items-center gap-6">
                <Loading variant={Some("primary".to_owned())} aria_label="Loading preview" />
                <Loading indicator="dots" variant={Some("secondary".to_owned())} aria_label="Loading messages" />
                <Loading indicator="bars" variant={Some("tertiary".to_owned())} aria_label="Loading assets" />
            </div>
    }
}
