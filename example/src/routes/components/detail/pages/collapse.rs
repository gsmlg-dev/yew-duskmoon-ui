use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Collapse;

const COLLAPSE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the collapse root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Native summary trigger and content markup rendered inside the details element.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as collapse-primary.",
    },
    ApiRow {
        prop: "open",
        ty: "bool",
        default: "false",
        docs: "Initial native disclosure state; the browser handles subsequent toggling.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, COLLAPSE_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Collapse;\n\nhtml! {\n    <Collapse variant={Some(\"primary\".to_owned())} open={true} class=\"collapse-card\">\n        <summary class=\"collapse-trigger\">\n            <span>{ \"Deployment details\" }</span>\n            <span class=\"collapse-icon\" aria-hidden=\"true\">{ \"v\" }</span>\n        </summary>\n        <div class=\"collapse-content\">\n            <p>{ \"Build, check, and publish steps are visible.\" }</p>\n        </div>\n    </Collapse>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Collapse variant={primary_variant()} open={true} class="collapse-card component-detail-demo-control">
            <summary class="collapse-trigger">
                <span>{ "Deployment details" }</span>
                <span class="collapse-icon" aria-hidden="true">{ "v" }</span>
            </summary>
            <div class="collapse-content">
                <p>{ "Use the summary to toggle this native disclosure with a pointer or keyboard." }</p>
            </div>
        </Collapse>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Collapse variant={variant(color)} open={true} class="component-detail-color-demo">
            <summary class="collapse-trigger">
                <span>{ color.label }</span>
                <span class="collapse-icon" aria-hidden="true">{ "v" }</span>
            </summary>
            <div class="collapse-content">
                <code>{ format!("collapse-{}", color.key) }</code>
            </div>
        </Collapse>
    }
}
