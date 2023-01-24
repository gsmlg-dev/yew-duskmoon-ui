use yew::prelude::*;
use yewdux::prelude::*;

use crate::states::config::ConfigStore;

use stylist::css;
use stylist::yew::use_style;

use yew_duskmoon::AppHeader;
use yew_duskmoon::Link;

use crate::routes::AppRoute;
use crate::theme::Theme;

/// Header component
#[function_component(Header)]
pub fn header() -> Html {
    let (state, _) = use_store::<ConfigStore>();
    let theme = use_context::<Theme>().expect("no theme found");
    let style = use_style(css!(
        r#"
        color: ${color};
        background-color: ${background_color};
        text-transform: ${text_transform};
    "#,
        color = theme.foreground,
        background_color = theme.background,
        text_transform = state.name,
    ));

    html! {
      <AppHeader
        classes={style}
        logo={
          html! {
            <h1>
              { "Yew Duskmoon UI" }
            </h1>
          }
        }
        menu={
          html! {
            <>
              <Link<AppRoute>
                to={AppRoute::Home}
              >
                { "Home" }
              </Link<AppRoute>>
              <Link<AppRoute>
                to={AppRoute::ComponentsRoot}
              >
                { "Components" }
              </Link<AppRoute>>
              <Link<AppRoute>
                to={AppRoute::About}
              >
                { "About" }
              </Link<AppRoute>>
            </>
          }
        }
      />
    }
}
