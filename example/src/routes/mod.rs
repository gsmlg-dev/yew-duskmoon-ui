use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod components;
pub mod icons;
pub mod home;

use self::components::switch_components;
use self::components::ComponentsRoute;
use self::icons::switch_icons;
use self::icons::IconsRoute;
use about::About;
use home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/components")]
    ComponentsRoot,
    #[at("/components/*")]
    Components,
    #[at("/icons")]
    IconsRoot,
    #[at("/icons/*")]
    Icons,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Components | AppRoute::ComponentsRoot => {
            html! { <Switch<ComponentsRoute> render={switch_components} /> }
        },
        AppRoute::Icons | AppRoute::IconsRoot => {
          html! { <Switch<IconsRoute> render={switch_icons} /> }
        },
        AppRoute::NotFound => html! { "Page not found" },
    }
}
