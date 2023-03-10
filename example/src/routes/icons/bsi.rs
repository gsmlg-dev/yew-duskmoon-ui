use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::Typography;
use yew_duskmoon::Card;
use stylist::css;
use stylist::yew::use_style;
use yew_duskmoon_icons::bsi_names::*;

/// Components page
#[function_component(BootstrapIcons)]
pub fn component() -> Html {
  let style = use_style(css!(
    r#"
    width: 83.4%;
    .icon-container {
      display: flex;
      flex-direction: row;
      flex-wrap: wrap;
      justify-content: flex-start;
      align-items: flex-start;
      gap: 2rem;
      width: 100%;
      padding: 0;
    }
    .icon {
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      width: calc(25% - 5em);
      height: 140px;
      box-shadow: 0 0 2px 4px rgb(233 233 233 / 40%);
      padding: 1.4em;
    }
    .t {
      font-size: 1.5em;
      font-weight: bold;
    }
    .c {
      background: #000;
      color: #fff;
      padding: 0.618em;
    }
    .t,.v,.c {
      display: flex;
    }
  "#
  ));

  html! {
    <div class="app">
      <div class="app-main">
        <Card 
          classes={ style }
          title={html!{
            <Typography level={TypographyLevel::H1}>
              {"Duskmoon Icons - Bootstrap Icons"}
            </Typography>
          }}
        >
          <div class="icon-container">
            {NAMES.into_iter().map(|n| {
              html!{
                <div class="icon">
                  <label class="t">{ format!("{}", n) }</label>
                  <div class="v">
                    <BSIcon name={n} size={AttrValue::from("32")} />
                  </div>
                  <pre class="c">
                    {format!("use yew_duskmoon_icons::bsi::BS_{};\n", n)}
                    {format!("html!{{ <BS_{} /> }}\n", n)}
                    {"use yew_duskmoon_icons::bsi_names::BSIcon;\n"}
                    {format!("html!{{ <BSIcon name=\"{}\" /> }}", n)}
                  </pre>
                </div>
              }
            }).collect::<Html>()}
          </div>
        </Card>
      </div>
    </div>
  }
}
