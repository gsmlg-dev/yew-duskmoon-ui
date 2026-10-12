use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::{Dropdown, DropdownPlacement};

const API: &[ApiRow] = &[
    ApiRow {
        prop: "id",
        ty: "AttrValue",
        default: "required",
        docs: "Unique native popover target.",
    },
    ApiRow {
        prop: "trigger_label",
        ty: "AttrValue",
        default: "required",
        docs: "Accessible button text.",
    },
    ApiRow {
        prop: "placement",
        ty: "DropdownPlacement",
        default: "BlockEnd",
        docs:
            "Logical BlockStart, BlockEnd, InlineStart, or InlineEnd with Core collision fallbacks.",
    },
    ApiRow {
        prop: "class / content_class / trigger_class",
        ty: "Classes",
        default: "empty / empty / btn btn-primary",
        docs: "Classes for the wrapper, surface, and trigger.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Surface content; Menu child markup is optional.",
    },
    ApiRow {
        prop: "disabled",
        ty: "bool",
        default: "false",
        docs: "Disables the trigger.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Dropdown;

html! {
    <Dropdown id="workspace-actions" trigger_label="Workspace actions">
        <a class="menu-item" href="#docs">{ "Documentation" }</a>
        <button type="button" class="menu-item">{ "Create workspace" }</button>
    </Dropdown>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="flex flex-wrap gap-4">
            <Dropdown id="demo-dropdown-end" trigger_label="Open actions">
                <a class="menu-item" href="#docs">{"Documentation"}</a>
                <a class="menu-item" href="#api">{"API reference"}</a>
            </Dropdown>
            <Dropdown id="demo-dropdown-start" trigger_label="Open above" placement={DropdownPlacement::BlockStart}>
                <a class="menu-item" href="#demo">{"Demo"}</a>
            </Dropdown>
        </div>
    }
}
