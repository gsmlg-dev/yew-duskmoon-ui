use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::Typography;
use yew_duskmoon::Card;
use stylist::css;
use stylist::yew::use_style;
use strum::IntoEnumIterator;
use yew_duskmoon_icons::bsi::*;

/// Components page
#[function_component(BootstrapIcons)]
pub fn component() -> Html {
  let style = use_style(css!(
    r#"
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    gap: 2rem;
    list-style: none;
    width: 100%;
    padding: 0;
    .t::after {
      content: ":";
      display: inline-flex;
      padding: 0 0.1em;
    }
    .t,.v,.c {
      display: flex;
    }
  "#
  ));

  html! {
    <div class="app">
      <div class="app-main">
      <Card title={ html!{
        <Typography level={TypographyLevel::H1}>
        {"Duskmoon Icons - Bootstrap Icons"}
        </Typography>
      }}>
        <div class={ style }>
          {BSIcon::iter().into_iter().map(|n| {
            html!{
              <div class="icon">
                  <label class="t">{ format!("{}", n) }</label>
                  <div class="v">
                    <@{format!("{}", n)} />
                  </div>
              </div>
            }
          }).collect::<Html>()}
        </div>
      </Card>
      </div>
    </div>
  }
}
