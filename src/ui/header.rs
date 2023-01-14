use yew::prelude::*;

use stylist::css;
use stylist::yew::use_style;

/// Props for [`Header`]
#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// logo part
    pub logo: Html,
    /// menu part
    #[prop_or_default]
    pub menu: Html,
    /// infor part
    #[prop_or_default]
    pub info: Html,
}

/// Header component
#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let style = use_style(css!(
        r#"
    display: flex;
    flex-direction: row;
    height: 60px;
    justify-content: space-between;
    padding: 0 2em;
    box-shadow: 0 1px 4px 1px rgba(38, 60, 86, 0.1);
    z-index: 1000;
    font-size: 14px;
    .left {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
        height: 100%;
    }
    .logo {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
        height: 100%;
    }
    .menu {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
        height: 100%;
        margin: 0 1.25em;
    }
    .right {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-end;
        height: 100%;
    }
    .info {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
        height: 100%;
    }
    "#
    ));
    let owned_props = props.clone();

    html! {
      <header class={classes!(style, owned_props.classes)}>
        <div
          class="left"
        >
          <div class="logo">
              { owned_props.logo }
          </div>
          <div class="menu">
              { owned_props.menu }
          </div>
        </div>
        <div class="right">
          <div class="info">
              { owned_props.info }
          </div>
        </div>
      </header>
    }
}
