use yew::prelude::*;
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

    html! {
      <div class="app">
        <div class="app-main">
          <Card title={ html! { "Duskmoon UI" } }>
              <div class="space">
                <div class="space-item">
                  <Button onclick={set_lower}>{ "captialize" }</Button>
                </div>
                <div class="space-item">
                  <Button classes="btn-primary" onclick={set_upper}>{ "UPPERCASE" }</Button>
                </div>
              </div>
          </Card>
        </div>
      </div>
    }
}
