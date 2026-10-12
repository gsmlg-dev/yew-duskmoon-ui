use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::BottomNavigation;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::BottomNavigation;\n\nhtml! {\n    <BottomNavigation\n        variant={Some(\"primary\".to_owned())}\n        class=\"bottom-nav bottom-nav-responsive\"\n    >\n        <a href=\"#home\" class=\"bottom-nav-item active\">\n            <span class=\"bottom-nav-indicator\"><span class=\"bottom-nav-icon\">{ \"H\" }</span></span>\n            <span class=\"bottom-nav-label\">{ \"Home\" }</span>\n        </a>\n        <a href=\"#search\" class=\"bottom-nav-item\">\n            <span class=\"bottom-nav-indicator\"><span class=\"bottom-nav-icon\">{ \"S\" }</span></span>\n            <span class=\"bottom-nav-label\">{ \"Search\" }</span>\n        </a>\n    </BottomNavigation>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <>
            { preview_style() }
            <BottomNavigation variant={primary_variant()} class="bottom-nav bottom-nav-responsive bottom-nav-bordered component-detail-bottom-navigation-preview">
                { nav_item("#home", "H", "Home", true, None) }
                { nav_item("#search", "S", "Search", false, None) }
                { nav_item("#inbox", "I", "Inbox", false, Some("3")) }
                <a href="#secondary" class="bottom-nav-item bottom-nav-item-active-secondary">
                    <span class="bottom-nav-indicator"><span class="bottom-nav-icon">{"S"}</span></span>
                    <span class="bottom-nav-label">{"Secondary"}</span>
                </a>
                <a href="#tertiary" class="bottom-nav-item bottom-nav-item-active-tertiary">
                    <span class="bottom-nav-indicator"><span class="bottom-nav-icon">{"T"}</span></span>
                    <span class="bottom-nav-label">{"Tertiary"}</span>
                </a>
            </BottomNavigation>
        </>
    }
}

fn nav_item(
    href: &'static str,
    icon: &'static str,
    label: &'static str,
    active: bool,
    badge: Option<&'static str>,
) -> Html {
    html! {
        <a href={href} class={classes!("bottom-nav-item", active.then_some("active"))}>
            <span class="bottom-nav-indicator">
                <span class="bottom-nav-icon">{ icon }</span>
                {
                    badge.map(|value| html! { <span class="bottom-nav-badge">{ value }</span> })
                        .unwrap_or_default()
                }
            </span>
            <span class="bottom-nav-label">{ label }</span>
        </a>
    }
}

fn preview_style() -> Html {
    html! {
        <style>
            { ".component-detail-bottom-navigation-preview.bottom-nav{position:relative;left:auto;right:auto;bottom:auto;z-index:auto;width:min(100%,34rem);min-height:4.75rem;padding-bottom:0;border:1px solid var(--dm-line);border-radius:8px;overflow:hidden}.component-detail-bottom-navigation-preview.bottom-nav .bottom-nav-item{min-width:0}" }
        </style>
    }
}
