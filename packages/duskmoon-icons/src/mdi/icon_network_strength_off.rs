#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_NetworkStrengthOff)]
pub fn r#icon_network_strength_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21,1L12.4,9.6L21,18.19M4.77,4.5L3.5,5.77L9.86,12.13L1,21H18.73L20.73,23L22,21.73" />
    </svg>
  }
}
