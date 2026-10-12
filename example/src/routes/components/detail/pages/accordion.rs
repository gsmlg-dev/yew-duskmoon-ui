use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Accordion;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the accordion root; use it for filled, outlined, separated, density, and animation classes.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Accordion items composed from accordion-item, accordion-header, title, icon, content, and body nodes.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as accordion-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Accordion;

html! {
    <Accordion variant={Some("primary".to_owned())} class="accordion-outlined">
        <details class="accordion-item" open=true>
            <summary class="accordion-header">
                <span class="accordion-title">{ "What is Duskmoon?" }</span>
                <span class="accordion-icon">{ "v" }</span>
            </summary>
            <div class="accordion-content">
                <div class="accordion-body">{ "A Material Design inspired UI system." }</div>
            </div>
        </details>
    </Accordion>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Accordion variant={primary_variant()} class="component-detail-accordion-demo accordion-outlined">
            <details class="accordion-item" open=true>
                <summary class="accordion-header">
                    <span class="accordion-title">{ "Usage guidance" }</span>
                    <span class="accordion-icon">{ "v" }</span>
                </summary>
                <div class="accordion-content">
                    <div class="accordion-body">
                        { "Open items show related content inline while keeping the page context visible." }
                    </div>
                </div>
            </details>
            <details class="accordion-item">
                <summary class="accordion-header">
                    <span class="accordion-title">{ "API details" }</span>
                    <span class="accordion-icon">{ "v" }</span>
                </summary>
                <div class="accordion-content">
                    <div class="accordion-body">
                        { "Closed items keep secondary content compact until the user expands it." }
                    </div>
                </div>
            </details>
        </Accordion>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Accordion variant={variant(color)} class="component-detail-color-demo">
            <details class="accordion-item" open=true>
                <summary class="accordion-header">
                    <span class="accordion-title">{ color.label }</span>
                    <span class="accordion-icon">{ "v" }</span>
                </summary>
                <div class="accordion-content">
                    <div class="accordion-body">{ format!("accordion-{}", color.key) }</div>
                </div>
            </details>
        </Accordion>
    }
}
