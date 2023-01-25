use stylist::css;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::Button;
use yew_duskmoon::Card;
use yewdux::prelude::*;

use crate::states::config::ConfigStore;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let (_state, dispatch) = use_store::<ConfigStore>();
    let set_lower = dispatch.reduce_mut_callback(|l| l.name = "capitalize".to_string());
    let set_upper = dispatch.reduce_mut_callback(|l| l.name = "uppercase".to_string());

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
        h1 {
          display: flex;
          font-size: 8rem;
          text-shadow: #FC0 1px 0 10px;
        }
      "#
    ));

    let style = use_style(css!(
        r#"
        width: 90%;
        .card-title {
          color: #4285f4;
        }
      "#
    ));

    html! {
      <div class="app">
        <div class={ hero_style }>
          <h1>
            { "Duskmoon UI" }
          </h1>
        </div>
        <div class="app-main">
          <Card title={ html! { <h4 class="card-title">{"Config Header Text Transform"}</h4> } } classes={ style }>
              <div class="space">
                <div class="space-item">
                  <Button onclick={set_lower}>{ "captialize" }</Button>
                </div>
                <div class="space-item">
                  <Button r#type={ButtonType::Primary} onclick={set_upper}>{ "UPPERCASE" }</Button>
                </div>
              </div>
          </Card>
        </div>
      </div>
    }
}
