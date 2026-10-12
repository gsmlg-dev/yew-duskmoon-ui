use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Validator;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Additional wrapper classes.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Native validator controls, hints, and errors.",
    },
    ApiRow {
        prop: "immediate",
        ty: "bool",
        default: "false",
        docs: "Show native valid/invalid styles before user interaction.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Optional form-group error, success, warning, or info state.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Validator;

html! {
    <Validator>
        <label class="form-label" for="validator-email">{ "Email" }</label>
        <input id="validator-email" class="input validator" type="email" required={true} placeholder="you@example.com" />
        <span class="validator-hint">{ "Enter a valid email, then move focus away." }</span>
        <span class="validator-error">{ "A valid email is required." }</span>
    </Validator>
}"#.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Validator>
                <label class="form-label" for="validator-email">{ "Email" }</label>
                <input id="validator-email" class="input validator" type="email" required={true} placeholder="you@example.com" />
                <span class="validator-hint">{ "Enter a valid email, then move focus away." }</span>
                <span class="validator-error">{ "A valid email is required." }</span>
            </Validator>
    }
}
