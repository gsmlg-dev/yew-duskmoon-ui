use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Swap;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "label",
        ty: "AttrValue",
        default: "required",
        docs: "Accessible toggle name independent of visual indicators.",
    },
    ApiRow {
        prop: "on / off",
        ty: "Html",
        default: "required",
        docs: "Non-interactive indicators hidden from assistive technology.",
    },
    ApiRow {
        prop: "pressed",
        ty: "bool",
        default: "false",
        docs: "Controlled semantic aria-pressed state.",
    },
    ApiRow {
        prop: "on_change",
        ty: "Callback<bool>",
        default: "empty",
        docs: "Requests the next pressed state; update pressed in the parent.",
    },
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Button styling and optional swap-rotate.",
    },
    ApiRow {
        prop: "disabled",
        ty: "bool",
        default: "false",
        docs: "Disables interaction.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Swap;

let muted = use_state(|| false);
let on_change = { let muted = muted.clone(); Callback::from(move |value| muted.set(value)) };
html! {
    <Swap label="Mute audio" pressed={*muted} {on_change}
        on={html! { "Muted" }} off={html! { "Sound on" }} class="btn btn-primary swap-rotate" />
}"##
    .to_owned()
}

#[function_component(SwapDemo)]
fn swap_demo() -> Html {
    let muted = use_state(|| false);
    let on_change = {
        let muted = muted.clone();
        Callback::from(move |value| muted.set(value))
    };
    html! { <Swap label="Mute audio" pressed={*muted} {on_change} on={html! {"Muted"}} off={html! {"Sound on"}} class="btn btn-primary swap-rotate" /> }
}

fn demo(_: &ComponentSpec) -> Html {
    html! { <SwapDemo /> }
}
