#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SubdirectoryArrowLeft)]
pub fn r#icon_subdirectory_arrow_left(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11,9L12.42,10.42L8.83,14H18V4H20V16H8.83L12.42,19.58L11,21L5,15L11,9Z" />
    </svg>
  }
}
