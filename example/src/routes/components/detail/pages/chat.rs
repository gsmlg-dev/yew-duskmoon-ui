use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Chat;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Root classes, including chat-start or chat-end alignment.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Native chat-header, chat-bubble, chat-footer, and chat-avatar slots.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs:
            "Semantic colors apply to native chat-bubble descendants; start/end apply to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Chat;

html! {
    <Chat variant={Some("primary".to_owned())} class="chat-start">
        <div class="chat-header">{ "Assistant" }</div>
        <div class="chat-bubble"><div class="chat-bubble-content">{ "Ready to help." }</div></div>
        <div class="chat-footer">{ "Just now" }</div>
    </Chat>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Chat variant={Some("primary".to_owned())} class="chat-start">
                <div class="chat-header">{ "Assistant" }</div>
                <div class="chat-bubble"><div class="chat-bubble-content">{ "Ready to help." }</div></div>
                <div class="chat-footer">{ "Just now" }</div>
            </Chat>
            <Chat variant={Some("secondary".to_owned())} class="chat-end">
                <div class="chat-header">{ "You" }</div>
                <div class="chat-bubble"><div class="chat-bubble-content">{ "Show me the updated components." }</div></div>
            </Chat>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Chat variant={variant(color)} class="chat-start">
            <div class="chat-bubble"><div class="chat-bubble-content">{ color.label }</div></div>
        </Chat>
    }
}
