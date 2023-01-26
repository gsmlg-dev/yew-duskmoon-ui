use stylist::css;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_duskmoon::Card;
use yew_duskmoon::Link;

use super::IconsRoute;

/// Icons page
#[function_component(IconsRoot)]
pub fn icons_root() -> Html {
  let hero_style = use_style(css!(
      r#"
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    min-height: 300px;
    background-color: #4285f4;
    color: #c2c5cc;
    user-select: none;
    background-image: url(./assets/moon.png);
    background-repeat: no-repeat;
    background-size: 20%;
    background-position: right;
    background-blend-mode: hard-light;
    h1 {
      display: flex;
      font-size: 8rem;
      text-shadow: #FC0 1px 0 10px;
    }
  "#
  ));
  let link_style = use_style(css!(
    r#"
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    .item {
      display: flex;
      justify-content: flex-start;
      height: 2.5rem;
    }
  "#
  ));
  html! {
    <div class="app">
      <div class={ hero_style }>
        <h1>
          { "Duskmoon Icons" }
        </h1>
      </div>
      <div class="app-main">
        <Card title={ html!{
          <h3> { "Icons" } </h3>
        }}>
          <div class={ link_style }>
            <div class="item">
              <Link<IconsRoute> to={IconsRoute::MaterialDesignIcons}>
                {"Material Design Icons"}
              </Link<IconsRoute>>
            </div>
            <div class="item">
              <Link<IconsRoute> to={IconsRoute::BootstrapIcons}>
                {"Bootstrap Icons"}
              </Link<IconsRoute>>
            </div>
          </div>
        </Card>
      </div>
    </div>
  }
}
