use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Hero;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Hero;

html! {
    <Hero variant={Some("start".to_owned())}><div class="hero-content p-6"><div><h3>{ "Build with DuskMoon" }</h3><p>{ "Compose accessible native controls in Yew." }</p><a class="btn btn-primary" href="#api">{ "Explore API" }</a></div></div></Hero>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Hero variant={Some("start".to_owned())}><div class="hero-content p-6"><div><h3>{ "Build with DuskMoon" }</h3><p>{ "Compose accessible native controls in Yew." }</p><a class="btn btn-primary" href="#api">{ "Explore API" }</a></div></div></Hero>
    }
}
