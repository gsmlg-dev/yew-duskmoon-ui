#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Leek)]
pub fn r#icon_leek(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10 2V5.55L12 7.55L14 5.55V2H10M6.34 4L3.87 6.5L15 17.62V12.67L6.34 4M17.66 4L13.06 8.61L15.54 11.09L20.13 6.5L17.66 4M9 13.74V20A2 2 0 0 0 11 22H13A2 2 0 0 0 15 20V19.74L13 17.74V20H11V15.74L9 13.74Z" />
    </svg>
  }
}
