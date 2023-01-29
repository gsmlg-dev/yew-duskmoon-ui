#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FlashAlert)]
pub fn r#icon_flash_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5,2V13H8V22L15,10H11L15,2M17,15H19V17H17V15M17,7H19V13H17V7Z" />
    </svg>
  }
}
