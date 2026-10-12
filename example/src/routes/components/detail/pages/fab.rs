use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Fab;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "id",
        ty: "AttrValue",
        default: "required",
        docs: "Unique native speed-dial target.",
    },
    ApiRow {
        prop: "trigger_label",
        ty: "AttrValue",
        default: "required",
        docs: "Accessible trigger text.",
    },
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "fab-start or fab-contained; default placement is fixed at the viewport end.",
    },
    ApiRow {
        prop: "trigger_class",
        ty: "Classes",
        default: "btn btn-primary",
        docs: "Button color/appearance and optional fab-extended.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "fab-action children paired with Button classes and optional fab-label.",
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
    r##"use yew_duskmoon::Fab;

html! {
    <Fab id="create-actions" trigger_label="Create">
        <div class="fab-action">
            <span class="fab-label">{ "New document" }</span>
            <button type="button" class="btn btn-primary">{ "Document" }</button>
        </div>
    </Fab>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="relative min-h-64 w-full">
            <Fab id="demo-fab" trigger_label="Create" class="fab-contained" trigger_class="btn btn-primary fab-extended">
                <div class="fab-action"><span class="fab-label">{"New document"}</span><button type="button" class="btn btn-primary" popovertarget="demo-fab" popovertargetaction="hide">{"Document"}</button></div>
                <div class="fab-action"><span class="fab-label">{"New folder"}</span><button type="button" class="btn btn-secondary" popovertarget="demo-fab" popovertargetaction="hide">{"Folder"}</button></div>
            </Fab>
        </div>
    }
}
