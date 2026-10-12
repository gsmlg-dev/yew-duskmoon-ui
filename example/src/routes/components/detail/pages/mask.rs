use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Mask;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Mask;

html! {
    <div class="flex gap-4"><Mask variant={Some("circle".to_owned())} class="bg-primary-container w-20 h-20 p-4">{ "DM" }</Mask><Mask variant={Some("hexagon".to_owned())} class="bg-secondary-container w-20 h-20 p-4">{ "UI" }</Mask></div>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="flex gap-4"><Mask variant={Some("circle".to_owned())} class="bg-primary-container w-20 h-20 p-4">{ "DM" }</Mask><Mask variant={Some("hexagon".to_owned())} class="bg-secondary-container w-20 h-20 p-4">{ "UI" }</Mask></div>
    }
}
