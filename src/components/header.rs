use yew::prelude::*;
use yew_router::prelude::*;

use stylist::css;
use stylist::yew::use_style;

use crate::routes::AppRoute;
use crate::theme::Theme;

/// Header component
#[function_component(Header)]
pub fn header() -> Html {
  let theme = use_context::<Theme>().expect("no theme found");
  let s = css!(
    r#"
        color: ${color};
        background-color: ${background_color};

        display: flex;
        flex-direction: row;
        height: 60px;
        display: flex;
        justify-content: space-between;
        padding: 0 2em;
        box-shadow: 0 1px 4px 1px rgba(38, 60, 86, 0.1);
        z-index: 1000;
        font-size: 14px;
    
        .page-header-title {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: flex-start;
        }
        .page-header-menu {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: flex-start;
        }
    "#,
    color = theme.foreground,
    background_color = theme.background,
  );
  let style = use_style(s);

  html! {
    <header class={style}>
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
