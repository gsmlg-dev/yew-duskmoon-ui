use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::ConsolePage;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::ConsolePage;

html! {
    <ConsolePage class="[--console-page-min-height:20rem]"><div class="console-page-frame"><header class="console-page-appbar appbar"><span class="appbar-title">{ "Workspace" }</span></header><aside class="console-page-sidebar"><div class="console-page-sidebar-header">{ "Projects" }</div><nav class="console-page-sidebar-body"><a class="link" href="#docs">{ "Overview" }</a></nav></aside><main class="console-page-main"><h3>{ "Project overview" }</h3><p>{ "Core owns the responsive sidebar layout." }</p></main></div></ConsolePage>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <ConsolePage class="[--console-page-min-height:20rem]"><div class="console-page-frame"><header class="console-page-appbar appbar"><span class="appbar-title">{ "Workspace" }</span></header><aside class="console-page-sidebar"><div class="console-page-sidebar-header">{ "Projects" }</div><nav class="console-page-sidebar-body"><a class="link" href="#docs">{ "Overview" }</a></nav></aside><section class="console-page-main"><h3>{ "Project overview" }</h3><p>{ "Core owns the responsive sidebar layout." }</p></section></div></ConsolePage>
    }
}
