use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Carousel;

const EXTRA_API: &[ApiRow] = &[ApiRow {
    prop: "aria_label",
    ty: "AttrValue",
    default: "Carousel",
    docs: "Accessible name of the keyboard-focusable scroll region.",
}];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
        .with_additional_api("Additional props", EXTRA_API)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Carousel;

html! {
    <Carousel aria_label="Release highlights"><section class="carousel-item card p-6"><h3>{ "Build" }</h3><p>{ "Compile the components." }</p></section><section class="carousel-item card p-6"><h3>{ "Verify" }</h3><p>{ "Exercise native browser behavior." }</p></section><section class="carousel-item card p-6"><h3>{ "Publish" }</h3><p>{ "Deliver a verified release." }</p></section></Carousel>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Carousel aria_label="Release highlights"><section class="carousel-item card p-6"><h3>{ "Build" }</h3><p>{ "Compile the components." }</p></section><section class="carousel-item card p-6"><h3>{ "Verify" }</h3><p>{ "Exercise native browser behavior." }</p></section><section class="carousel-item card p-6"><h3>{ "Publish" }</h3><p>{ "Deliver a verified release." }</p></section></Carousel>
    }
}
