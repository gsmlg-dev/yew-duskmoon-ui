use yew::prelude::*;
use yew_router::prelude::*;

use self::button::ButtonComponent;
use crate::routes::AppRoute;

pub mod button;
pub mod root;
pub use self::root::ComponentsRoot;

#[derive(Clone, Routable, PartialEq)]
pub enum ComponentsRoute {
    #[at("/components")]
    ComponentsRoot,
    #[at("/components/button")]
    ButtonComponent,
    #[not_found]
    #[at("/components/404")]
    NotFound,
}

/// Switch components routes
pub fn switch_components(route: ComponentsRoute) -> Html {
    match route {
        ComponentsRoute::ComponentsRoot => html! { <ComponentsRoot /> },
        ComponentsRoute::ButtonComponent => html! {<ButtonComponent />},
        ComponentsRoute::NotFound => html! {<Redirect<AppRoute> to={AppRoute::NotFound}/>},
    }
}
