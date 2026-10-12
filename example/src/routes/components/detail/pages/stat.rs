use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Stat;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Stat;

html! {
    <div class="stats stats-vertical"><Stat variant={Some("primary".to_owned())}><span class="stat-title">{ "Published components" }</span><span class="stat-value">{ "84" }</span><span class="stat-desc">{ "CSS contracts supplied by Core" }</span></Stat><Stat><span class="stat-title">{ "Build status" }</span><span class="stat-value">{ "Ready" }</span></Stat></div>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="stats stats-vertical"><Stat variant={Some("primary".to_owned())}><span class="stat-title">{ "Published components" }</span><span class="stat-value">{ "84" }</span><span class="stat-desc">{ "CSS contracts supplied by Core" }</span></Stat><Stat><span class="stat-title">{ "Build status" }</span><span class="stat-value">{ "Ready" }</span></Stat></div>
    }
}
