#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ProjectorScreenOff)]
pub fn r#icon_projector_screen_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 14.89V16.59L17.21 20.79L15.79 22.21L13 19.41V22H11V19.41L8.21 22.21L6.79 20.79L11 16.59V14H5V6.89L1.11 3L2.39 1.73L22.11 21.46L20.84 22.73L13 14.89M19 14V5H20C20.55 5 21 4.55 21 4V3C21 2.45 20.55 2 20 2H5.2L17.2 14H19Z" />
    </svg>
  }
}
