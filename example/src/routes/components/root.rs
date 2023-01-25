use stylist::css;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_duskmoon::Card;
use yew_duskmoon::Link;

use super::ComponentsRoute;

/// Components page
#[function_component(ComponentsRoot)]
pub fn components_root() -> Html {
  let hero_style = use_style(css!(
      r#"
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    min-height: 300px;
    background-color: #4285f4;
    color: #c2c5cc;
    user-select none;
    h1 {
      display: flex;
      font-size: 8rem;
      text-shadow: #FC0 1px 0 10px;
    }
  "#
  ));
  html! {
    <div class="app">
      <div class={ hero_style }>
        <h1>
          { "Duskmoon Components" }
        </h1>
      </div>
      <div class="app-main">
        <Card title={ html!{
          <h3> { "General Components" } </h3>
        }}>
          <div>
            <div>
              <Link<ComponentsRoute> to={ComponentsRoute::ButtonComponent}>
                {"Button"}
              </Link<ComponentsRoute>>
            </div>
          </div>
        </Card>
      </div>
    </div>
  }
}
