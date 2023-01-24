use yew::prelude::*;
use yew_duskmoon::Button;
use yew_duskmoon::Card;

/// Components page
#[function_component(ButtonComponent)]
pub fn component() -> Html {
    html! {
      <div class="app">
        <div class="app-main">
          <Card title={ html!{ "Duskmoon Components - Button" } }>
            { "Button" }
            <div>
                <Button>{ "Button default" }</Button>
            </div>
          </Card>
        </div>
      </div>
    }
}
