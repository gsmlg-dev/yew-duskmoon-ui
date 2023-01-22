use yew::prelude::*;
use yewdux::prelude::*;
use yew_duskmoon::Card;
use yew_duskmoon::Button;

use crate::states::config::ConfigStore;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let (state, dispatch) = use_store::<ConfigStore>();
    let set_lower = dispatch.reduce_mut_callback(|l| l.name = "yew app sample".to_string());
    let set_upper = dispatch.reduce_mut_callback(|l| l.name = "YEW APP SAMPLE".to_string());

    html! {
      <div class="app">
        <div class="app-main">
          <Card title={ html! { state.name.clone() } }>
              <div class="space">
                <div class="space-item">
                  <Button onclick={set_lower}>{ "set lower case" }</Button>
                </div>
                <div class="space-item">
                  <Button classes="btn-primary" onclick={set_upper}>{ "SET UPPER CASE" }</Button>
                </div>
              </div>
          </Card>
        </div>
      </div>
    }
}
