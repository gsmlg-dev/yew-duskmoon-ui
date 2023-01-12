use yew::prelude::*;
// use yew_hooks::prelude::*;
use yewdux::prelude::*;

use crate::states::config::ConfigStore;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
  let (state, dispatch) = use_store::<ConfigStore>();
  let set_lower = dispatch.reduce_mut_callback(|l| l.value = "yew app sample".to_string() );
  let set_upper = dispatch.reduce_mut_callback(|l| l.value = "YEW APP SAMPLE".to_string() );

  html! {
    <div class="app">
      <div class="app-main">
        <div class="card">
          <div class="card-body">
            <h3>
              { state.value.clone() }
            </h3>
            <div class="space">
              <div class="space-item">
                <button class="btn btn-default" onclick={set_lower}>{ "set lower case" }</button>
              </div>
              <div class="space-item">
                <button class="btn btn-primary" onclick={set_upper}>{ "SET UPPER CASE" }</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  }
}
