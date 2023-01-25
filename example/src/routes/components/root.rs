use yew::prelude::*;
use yew_duskmoon::Card;

use super::ComponentsRoute;
use yew_duskmoon::Link;

/// Components page
#[function_component(ComponentsRoot)]
pub fn components_root() -> Html {
    html! {
      <div class="app">
        <div class="app-main">
          <Card title={ html!{
            <h3> { "Duskmoon Degin UI Components" } </h3>
          }}>
            <h4> { "Basic Components" } </h4>
            <p>
              <Link<ComponentsRoute> to={ComponentsRoute::ButtonComponent}>
                {"Button Component"}
              </Link<ComponentsRoute>>
            </p>
          </Card>
        </div>
      </div>
    }
}
