use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::prelude::*;

use yew_duskmoon::button::ButtonType;
use yew_duskmoon::Button;
use yew_duskmoon::Card;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    let state = use_async(async move { fetch_repo(("yew-duskmoon-ui".to_string()).clone()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };
    {
        // let state = state.clone();
        use_effect_with_deps(
            move |_| {
                // state.run();
            },
            (),
        );
    }
    
    html! {
      <div class="app">
        <div class="app-main">
          <Card>
              <div>
                <Button r#type={ButtonType::Primary} onclick={onclick}>{ "Load Repo Information" }</Button>
              </div>
              <div>
                {
                  if state.loading {
                    html! { "Loading, wait a sec..." }
                  } else {
                    if let Some(repo) = &state.data {
                      html! {
                        <div key={repo.id} class="space-h">
                            <div>
                              <label>{ "repo name: " }</label>
                              <b>{ &repo.name }</b>
                            </div>
                            <div>
                              <label>{ "repo web url: " }</label>
                              <b>{ &repo.html_url }</b>
                            </div>
                            <div>
                                <label>{ "Go to Github Repo: " }</label>
                                <Button
                                    r#type={ ButtonType::Link }
                                    href={ repo.html_url.clone() }
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    { &repo.name }
                                </Button>
                            </div>
                        </div>
                      }
                    } else if let Some(error) = &state.error {
                      match error {
                          Error::DeserializeError => html! { "DeserializeError" },
                          Error::RequestError => html! { "RequestError" },
                      }
                    } else {
                        html! {}
                    }
                  }
                }
              </div>
          </Card>
        </div>
      </div>
    }
}

async fn fetch_repo(repo: String) -> Result<Repo, Error> {
    let url = format!("https://api.github.com/repos/gsmlg-dev/{}", repo);
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        if let Ok(repo) = data.json::<Repo>().await {
            Ok(repo)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Repo {
    id: i32,
    name: String,
    html_url: String,
    created_at: String,
    pushed_at: String,
    size: i32,
    stargazers_count: i32,
    watchers_count: i32,
    forks_count: i32,
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}
