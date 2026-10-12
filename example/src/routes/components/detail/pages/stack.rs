use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Stack;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Stack;

html! {
    <Stack variant={Some("end".to_owned())}><div class="card bg-primary-container p-6">{ "Current release" }</div><div class="card bg-secondary-container p-6">{ "Previous release" }</div><div class="card bg-tertiary-container p-6">{ "Archived release" }</div></Stack>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Stack variant={Some("end".to_owned())}><div class="card bg-primary-container p-6">{ "Current release" }</div><div class="card bg-secondary-container p-6">{ "Previous release" }</div><div class="card bg-tertiary-container p-6">{ "Archived release" }</div></Stack>
    }
}
