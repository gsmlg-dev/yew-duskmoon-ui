use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::HomePage;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::HomePage;

html! {
    <HomePage class="[--home-page-min-height:20rem]"><div class="home-page-frame"><header class="home-page-header appbar"><span class="appbar-title">{ "DuskMoon" }</span><nav class="home-page-nav"><a class="link" href="#docs">{ "Docs" }</a></nav></header><main class="home-page-main"><div class="home-page-content"><h3>{ "Welcome" }</h3><p>{ "Use Core's page regions to build a public application." }</p></div></main><footer class="home-page-footer p-4">{ "Built with Yew" }</footer></div></HomePage>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <HomePage class="[--home-page-min-height:20rem]"><div class="home-page-frame"><header class="home-page-header appbar"><span class="appbar-title">{ "DuskMoon" }</span><nav class="home-page-nav"><a class="link" href="#docs">{ "Docs" }</a></nav></header><section class="home-page-main"><div class="home-page-content"><h3>{ "Welcome" }</h3><p>{ "Use Core's page regions to build a public application." }</p></div></section><footer class="home-page-footer p-4">{ "Built with Yew" }</footer></div></HomePage>
    }
}
