use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Select;

const SELECT_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the native select.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Native option/optgroup children; text-only legacy content becomes one option.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a select color class such as select-primary.",
    },
    ApiRow {
        prop: "id / name / aria_label",
        ty: "AttrValue",
        default: "empty",
        docs: "Native identity and accessible name.",
    },
    ApiRow {
        prop: "value",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional native selected value.",
    },
    ApiRow {
        prop: "disabled / required / multiple",
        ty: "bool",
        default: "false",
        docs: "Native select state and multiple selection.",
    },
    ApiRow {
        prop: "onchange",
        ty: "Callback<Event>",
        default: "noop",
        docs: "Native selection change events.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, SELECT_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Select;

html! {
    <label class="select-container">
        <span class="select-label">{ "Environment" }</span>
        <Select variant={Some("primary".to_owned())} name="environment">
            <option value="production">{ "Production" }</option>
            <option value="staging">{ "Staging" }</option>
        </Select>
    </label>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="select-container">
            <label class="select-label" for="select-environment">{ "Environment" }</label>
            <Select id="select-environment" name="environment" variant={primary_variant()} class="select-outlined">
                <option value="production">{ "Production" }</option>
                <option value="staging">{ "Staging" }</option>
            </Select>
            <span class="select-helper">{ "Choose where this release will deploy." }</span>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Select aria_label={color.label} variant={variant(color)} class="color-input">
            <option>{ color.label }</option>
            <option>{ "Other option" }</option>
        </Select>
    }
}
