use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Input;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class / variant",
        ty: "Classes / Option<String>",
        default: "empty / None",
        docs: "Native input classes and Core semantic color.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs:
            "Text becomes the native value; existing nested input markup retains composition mode.",
    },
    ApiRow {
        prop: "id / name / aria_label / placeholder",
        ty: "AttrValue",
        default: "empty",
        docs: "Native identity, accessible name, and placeholder.",
    },
    ApiRow {
        prop: "input_type",
        ty: "Option<AttrValue>",
        default: "text",
        docs: "Native input type.",
    },
    ApiRow {
        prop: "value",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional controlled native input value.",
    },
    ApiRow {
        prop: "required / disabled / readonly",
        ty: "bool",
        default: "false",
        docs: "Native input state.",
    },
    ApiRow {
        prop: "oninput / onchange",
        ty: "Callback",
        default: "noop",
        docs: "Native input and change events.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}
fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Input;

html! {
    <label class="form-group">
        <span class="form-label">{ "Repository name" }</span>
        <Input variant={Some("primary".to_owned())} name="repository" placeholder="Enter a name" />
    </label>
}"#
    .to_owned()
}
fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <label class="form-group"><span class="form-label">{ "Repository name" }</span>
                <Input variant={Some("primary".to_owned())} class="input-bordered" name="repository" placeholder="Enter a name" />
            </label>
            <label class="form-group"><span class="form-label">{ "Email" }</span>
                <Input variant={Some("success".to_owned())} input_type="email" name="email" placeholder="you@example.com" />
            </label>
        </div>
    }
}
fn color_variant(color: PaletteColor) -> Html {
    html! { <label class="form-group"><span class="form-label">{ color.label }</span>
        <Input variant={variant(color)} placeholder={color.label} />
    </label> }
}
