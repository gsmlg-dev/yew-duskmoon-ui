#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SmokeDetectorAlertOutline)]
pub fn r#icon_smoke_detector_alert_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17 3H3C1.9 3 1 3.9 1 5V19C1 20.1 1.9 21 3 21H17C18.1 21 19 20.1 19 19V5C19 3.9 18.1 3 17 3M17 19H3V5H17V19M10 18C13.3 18 16 15.3 16 12C16 8.7 13.3 6 10 6C6.7 6 4 8.7 4 12C4 15.3 6.7 18 10 18M10 8C12.2 8 14 9.8 14 12S12.2 16 10 16 6 14.2 6 12 7.8 8 10 8M23 7H21V13H23V8M23 15H21V17H23V15Z" />
    </svg>
  }
}
