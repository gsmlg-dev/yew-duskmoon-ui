use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::SegmentControl;

const SEGMENT_CONTROL_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the segmented control root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Segment buttons rendered inside the control.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a segment-control color class such as segment-control-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, SEGMENT_CONTROL_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::SegmentControl;\n\nhtml! {\n    <SegmentControl variant={Some(\"primary\".to_owned())}>\n        <button class=\"segment-item segment-item-active\">{ \"Day\" }</button>\n        <button class=\"segment-item\">{ \"Week\" }</button>\n        <button class=\"segment-item\">{ \"Month\" }</button>\n    </SegmentControl>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <SegmentControl variant={primary_variant()} class="segment-control-full">
            <button class="segment-item segment-item-active">{ "Preview" }</button>
            <button class="segment-item">{ "API" }</button>
            <button class="segment-item">{ "Code" }</button>
        </SegmentControl>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <SegmentControl variant={variant(color)}>
            <button class="segment-item segment-item-active">
                { color.label }
            </button>
            <button class="segment-item">{ "Docs" }</button>
        </SegmentControl>
    }
}
