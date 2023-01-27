use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::Typography;
use yew_duskmoon::Card;
use stylist::css;
use stylist::yew::use_style;
use yew_duskmoon_icons::mdi::*;

/// Components page
#[function_component(MaterialDesignIcons)]
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
      width: 140px;
    }
    .t,.v,.c {
      display: flex;
      justify-content: center;
      align-items: center;
    }
  "#
  ));

  html! {
    <div class="app">
      <div class="app-main">
        <Card
          classes={ style }
          title={ html!{
            <Typography level={TypographyLevel::H1}>
              {"Duskmoon Icons - Material Design Icons"}
            </Typography>
          }}
        >
          <div class="icon-container">
            {MDI_NAMES.into_iter().map(|n| {
              html!{
                <div class="icon">
                  <label class="t">{ format!("{}", n) }</label>
                  <div class="v">
                    <MDIcon name={n} size={32} />
                  </div>
                  <div class="c">
                    {format!("html!{{ <MD_{} /> }}", n)}
                  </div>
                  <div class="c">
                    {format!("html!{{ <MDIcon name=\"{}\" /> }}", n)}
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
