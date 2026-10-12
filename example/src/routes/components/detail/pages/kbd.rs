use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Kbd;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Kbd;

html! {
    <div class="flex flex-wrap items-center gap-2"><Kbd variant={Some("primary".to_owned())}>{ "Ctrl" }</Kbd><span>{ "+" }</span><Kbd>{ "K" }</Kbd></div>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="flex flex-wrap items-center gap-2"><Kbd variant={Some("primary".to_owned())}>{ "Ctrl" }</Kbd><span>{ "+" }</span><Kbd>{ "K" }</Kbd></div>
    }
}
