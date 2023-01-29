#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_BeakerAlertOutline)]
pub fn r#icon_beaker_alert_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 3H21V5C19.9 5 19 5.9 19 7V19C19 20.11 18.11 21 17 21H7C5.9 21 5 20.11 5 19V7C5 5.9 4.11 5 3 5V3M7 5V7H12V8H7V9H10V10H7V11H10V12H7V13H12V14H7V15H10V16H7V19H17V5H7M21 13V7H23V13H21M21 17V15H23V17H21Z" />
    </svg>
  }
}
