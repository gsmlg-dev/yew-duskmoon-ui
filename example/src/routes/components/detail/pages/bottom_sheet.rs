use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::BottomSheet;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "id",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Unique native dialog or popover target.",
    },
    ApiRow {
        prop: "modal",
        ty: "bool",
        default: "true",
        docs: "Modal dialog uses show-modal/close commands; false uses native auto popover.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Accessible surface name.",
    },
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Core position and size modifiers.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Header, content, and footer markup using bottomsheet classes.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Legacy modifier escape hatch; use supported Core size variants.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::BottomSheet;

html! {
    <>
        <button type="button" class="btn btn-primary" command="show-modal" commandfor="actions-sheet">{ "Open actions" }</button>
        <BottomSheet id="actions-sheet" aria_label="Actions" class="bottomsheet-sm">
            <div class="bottomsheet-header">
                <h2 class="bottomsheet-title">{ "Actions" }</h2>
                <button type="button" class="bottomsheet-close" aria-label="Close actions" command="close" commandfor="actions-sheet">{ "x" }</button>
            </div>
            <div class="bottomsheet-content">{ "Focused task content" }</div>
        </BottomSheet>
    </>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    surface("demo-bottomsheet".into(), "Quick actions", "primary")
}

fn surface(id: AttrValue, label: &'static str, color: &'static str) -> Html {
    html! {
        <>
            <button type="button" class={classes!("btn", format!("btn-{color}"))} command="show-modal" commandfor={id.clone()}>{label}</button>
            <BottomSheet id={id.clone()} aria_label={label} class="bottomsheet-sm">
                <div class="bottomsheet-header">
                    <h2 class="bottomsheet-title">{label}</h2>
                    <button type="button" class="bottomsheet-close" aria-label="Close actions" command="close" commandfor={id.clone()}>{"x"}</button>
                </div>
                <div class="bottomsheet-content"><p>{"The browser manages modality, focus, and Escape."}</p></div>
                <div class="bottomsheet-footer">
                    <button type="button" class="btn btn-text" command="close" commandfor={id}>{"Done"}</button>
                </div>
            </BottomSheet>
        </>
    }
}
