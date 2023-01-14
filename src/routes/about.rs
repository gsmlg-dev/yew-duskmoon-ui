use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::prelude::*;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    let state = use_async(async move { fetch_org(("gsmlg-dev".to_string()).clone()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                state.run();
            },
            (),
        );
    }

    html! {
      <div class="app">
        <div class="app-main">
          <div class="card">
            <div class="card-body">
              <div>
                <button class="btn btn-primary" {onclick}>{ "Load org of gsmlg-dev" }</button>
              </div>
              <div>
                {
                  if state.loading {
                    html! { "Loading, wait a sec..." }
                  } else {
                    html! {}
                  }
                }
              </div>
              {
                if let Some(org) = &state.data {
                  html! {
                    <div key={org.id} class="space-h">
                        <div>{ "Org login: " }<b>{ &org.login }</b></div>
                        <div>{ "Org web url: " }<b>{ &org.html_url }</b></div>
                        <div>
                            { "Go to Github Orginatizion: " }
                            <a
                                class="btn btn-link"
                                href={ org.html_url.clone() }
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                { "GSMLG-DEV Github" }
                            </a>
                        </div>
                    </div>
                  }
                } else {
                  html! {}
                }
              }
              <p>
                  {
                      if let Some(error) = &state.error {
                          match error {
                              Error::DeserializeError => html! { "DeserializeError" },
                              Error::RequestError => html! { "RequestError" },
                          }
                      } else {
                          html! {}
                      }
                  }
              </p>
            </div>
          </div>
        </div>
      </div>
    }
}

async fn fetch_org(repo: String) -> Result<Org, Error> {
    let url = format!("https://api.github.com/orgs/{}", repo);
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        if let Ok(org) = data.json::<Org>().await {
            Ok(org)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Org {
    id: i32,
    login: String,
    html_url: String,
    created_at: String,
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::About;
    use yew::start_app;

    #[wasm_bindgen_test]
    fn about_page_has_an_app_link() {
        start_app::<About>();

        let app_links = gloo_utils::document().get_elements_by_class_name("btn-link");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "GSMLG-DEV Github");
    }
}
