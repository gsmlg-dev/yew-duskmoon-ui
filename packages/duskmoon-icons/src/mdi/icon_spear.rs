#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Spear)]
pub fn r#icon_spear(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16 9H16.41L3.41 22L2 20.59L15 7.59V9H16M16 4V8H20L22 2L16 4Z" />
    </svg>
  }
}
