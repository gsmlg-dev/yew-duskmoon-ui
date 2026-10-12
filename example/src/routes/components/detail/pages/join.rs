use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Join;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Join;

html! {
    <Join><button class="join-item btn btn-outlined" type="button">{ "Previous" }</button><button class="join-item btn btn-primary" type="button">{ "Current" }</button><button class="join-item btn btn-outlined" type="button">{ "Next" }</button></Join>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Join><button class="join-item btn btn-outlined" type="button">{ "Previous" }</button><button class="join-item btn btn-primary" type="button">{ "Current" }</button><button class="join-item btn btn-outlined" type="button">{ "Next" }</button></Join>
    }
}
