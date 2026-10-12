use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Megamenu;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "id",
        ty: "AttrValue",
        default: "required",
        docs: "Unique native panel target and CSS anchor identity.",
    },
    ApiRow {
        prop: "trigger_label",
        ty: "AttrValue",
        default: "required",
        docs: "Accessible navigation and trigger label.",
    },
    ApiRow {
        prop: "class / panel_class",
        ty: "Classes",
        default: "empty",
        docs: "Wrapper and panel modifiers, including megamenu-panel-full. Compose megamenu-desktop with separate megamenu-mobile inline navigation.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "megamenu-grid groups with headings and Menu/Link children.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Megamenu;

html! {
    <>
        <Megamenu id="products-menu" trigger_label="Products" class="megamenu-desktop">
            <div class="megamenu-grid">
                <section class="megamenu-group">
                    <h2 class="megamenu-heading">{ "Libraries" }</h2>
                    <ul class="menu"><li><a class="link" href="#docs">{ "Yew components" }</a></li></ul>
                </section>
            </div>
        </Megamenu>
        <nav class="megamenu-mobile" aria-label="Products">
            <ul class="menu menu-vertical">
                <li><a class="link" href="#docs">{ "Yew components" }</a></li>
            </ul>
        </nav>
    </>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    // TODO(upstream): duskmoon-dev/duskmoonui#75
    // WORKAROUND(upstream): duskmoon-dev/duskmoonui#75 — use Core responsive inline navigation on mobile.
    html! {
        <>
            <Megamenu id="demo-megamenu" trigger_label="Explore DuskMoon" class="megamenu-desktop">
                <div class="megamenu-grid">
                    <section class="megamenu-group">
                        <h2 class="megamenu-heading">{"Components"}</h2>
                        <ul class="menu"><li><a class="link" href="#docs">{"Usage guide"}</a></li><li><a class="link" href="#api">{"API reference"}</a></li></ul>
                    </section>
                    <section class="megamenu-group">
                        <h2 class="megamenu-heading">{"Resources"}</h2>
                        <ul class="menu"><li><a class="link" href="https://github.com/duskmoon-dev/yew-duskmoon-ui">{"Source repository"}</a></li></ul>
                        <p class="megamenu-supporting">{"Native disclosure with browser light-dismiss and CSS anchors."}</p>
                    </section>
                </div>
            </Megamenu>
            <nav class="megamenu-mobile" aria-label="Explore DuskMoon">
                <ul class="menu menu-vertical">
                    <li><a class="link" href="#docs">{"Usage guide"}</a></li>
                    <li><a class="link" href="#api">{"API reference"}</a></li>
                    <li><a class="link" href="https://github.com/duskmoon-dev/yew-duskmoon-ui">{"Source repository"}</a></li>
                </ul>
            </nav>
        </>
    }
}
