use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Footer;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Footer;

html! {
    <Footer><nav aria-label="Project links"><h3 class="footer-title">{ "Project" }</h3><a class="link" href="#docs">{ "Documentation" }</a><a class="link" href="#api">{ "API" }</a></nav><section><h3 class="footer-title">{ "DuskMoon" }</h3><p>{ "Rust components with shared CSS contracts." }</p></section></Footer>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Footer><nav aria-label="Project links"><h3 class="footer-title">{ "Project" }</h3><a class="link" href="#docs">{ "Documentation" }</a><a class="link" href="#api">{ "API" }</a></nav><section><h3 class="footer-title">{ "DuskMoon" }</h3><p>{ "Rust components with shared CSS contracts." }</p></section></Footer>
    }
}
