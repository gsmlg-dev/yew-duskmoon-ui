use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::SidebarLayout;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::SidebarLayout;

html! {
    <SidebarLayout><aside class="sidebar-layout-sidebar p-4"><nav aria-label="Sidebar"><a class="link" href="#docs">{ "Documentation" }</a></nav></aside><section class="sidebar-layout-content p-6"><h3>{ "Content" }</h3><p>{ "The sidebar appears when the containing width allows it." }</p></section></SidebarLayout>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <SidebarLayout><aside class="sidebar-layout-sidebar p-4"><nav aria-label="Sidebar"><a class="link" href="#docs">{ "Documentation" }</a></nav></aside><section class="sidebar-layout-content p-6"><h3>{ "Content" }</h3><p>{ "The sidebar appears when the containing width allows it." }</p></section></SidebarLayout>
    }
}
