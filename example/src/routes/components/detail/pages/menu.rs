use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Menu;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Core layout modifiers such as menu-vertical or menu-compact.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Menu title and buttons or links with menu-item; active uses active or aria-current.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs:
            "Legacy modifier escape hatch; Core menu uses shared surface and primary active colors.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Menu;

html! {
    <Menu class="menu-vertical">
        <div class="menu-title">{ "Workspace" }</div>
        <a class="menu-item" href="#overview" aria-current="page">{ "Overview" }</a>
        <button class="menu-item" type="button">{ "Invite members" }</button>
        <button class="menu-item" type="button" disabled={true}>{ "Archived" }</button>
    </Menu>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Menu class="menu-vertical">
            <div class="menu-title">{"Actions"}</div>
            <a class="menu-item" href="#docs" aria-current="page">{"Open docs"}</a>
            <a class="menu-item" href="#api">{"View API"}</a>
            <a class="menu-item" href="#colors">{"Color matrix"}</a>
            <button class="menu-item" type="button" disabled={true}>{"Archived"}</button>
        </Menu>
    }
}
