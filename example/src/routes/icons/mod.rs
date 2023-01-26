use yew::prelude::*;
use yew_router::prelude::*;

use self::bsi::BootstrapIcons;
use self::mdi::MaterialDesignIcons;
use crate::routes::AppRoute;

pub mod mdi;
pub mod root;
pub mod bsi;
pub use self::root::IconsRoot;

#[derive(Clone, Routable, PartialEq)]
pub enum IconsRoute {
    #[at("/icons")]
    IconsRoot,
    #[at("/icons/mdi")]
    MaterialDesignIcons,
    #[at("/icons/bsi")]
    BootstrapIcons,
    #[not_found]
    #[at("/icons/404")]
    NotFound,
}

/// Switch icons routes
pub fn switch_icons(route: IconsRoute) -> Html {
    match route {
        IconsRoute::IconsRoot => html! { <IconsRoot /> },
        IconsRoute::MaterialDesignIcons => html! { <MaterialDesignIcons /> },
        IconsRoute::BootstrapIcons => html! { <BootstrapIcons /> },
        IconsRoute::NotFound => html! { <Redirect<AppRoute> to={AppRoute::NotFound} /> },
    }
}
