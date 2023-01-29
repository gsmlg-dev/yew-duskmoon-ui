#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FridgeIndustrialAlertOutline)]
pub fn r#icon_fridge_industrial_alert_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8 15H6V10H8V15M17 4V19C17 20.11 16.11 21 15 21V22H13V21H7V22H5V21C3.9 21 3 20.11 3 19V4C3 2.9 3.9 2 5 2H15C16.11 2 17 2.9 17 4M15 4H5V19H15V4M19 17H21V15H19V17M19 7V13H21V7H19Z" />
    </svg>
  }
}
