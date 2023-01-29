#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_WifiStrength4Alert)]
pub fn r#icon_wifi_strength_4_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 3C7.8 3 3.7 4.4 .4 7C4.4 12.1 7.9 16.4 12 21.5C14.4 18.5 16.7 15.7 19 12.8V8H22.8C23 7.7 23.4 7.3 23.6 7C20.3 4.4 16.2 3 12 3M21 10V16H23V10M21 18V20H23V18" />
    </svg>
  }
}
