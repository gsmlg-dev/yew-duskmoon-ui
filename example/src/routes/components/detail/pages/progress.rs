use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Progress;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class / variant",
        ty: "Classes / Option<String>",
        default: "empty / None",
        docs: "Core progress classes and semantic color.",
    },
    ApiRow {
        prop: "value / max",
        ty: "Option<f64> / f64",
        default: "None / 100",
        docs: "Native progress values; omitted value is indeterminate.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "AttrValue",
        default: "Progress",
        docs: "Accessible progressbar name.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Optional custom track composition; empty children render native progress.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Progress;

html! {
    <Progress variant={Some("primary".to_owned())} value={68.0} max={100.0} aria_label="Upload progress" />
}"#.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <span>{ "Upload progress: 68%" }</span>
            <Progress variant={primary_variant()} value={68.0} aria_label="Upload progress" />
            <span>{ "Preparing files" }</span>
            <Progress variant={primary_variant()} aria_label="Preparing files" />
        </div>
    }
}
