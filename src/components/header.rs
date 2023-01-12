use yew::prelude::*;
use yew_router::prelude::*;
use yew::{classes, html};

use crate::routes::AppRoute;

/// Header component
#[function_component(Header)]
pub fn header() -> Html {
  html! {
    <header class={classes!("page-header")}>
      <div
        class="page-header-title"
      >
        <h1>
        { "Yew APP Sample" }
        </h1>
      </div>
      <div class="page-header-menu">
        <div class="page-header-menu-item">
          <Link<AppRoute>
            to={AppRoute::Home}
            classes="btn btn-link"
          >
            { "Home" }
          </Link<AppRoute>>
        </div>
        <div class="page-header-menu-item">
          <Link<AppRoute>
            to={AppRoute::About}
            classes="btn btn-link"
          >
            { "About" }
          </Link<AppRoute>>
        </div>
      </div>
    </header>
  }
}