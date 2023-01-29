#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BorderHorizontal)]
pub fn r#icon_border_horizontal(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19,21H21V19H19M15,21H17V19H15M11,17H13V15H11M19,9H21V7H19M19,5H21V3H19M3,13H21V11H3M11,21H13V19H11M19,17H21V15H19M13,3H11V5H13M13,7H11V9H13M17,3H15V5H17M9,3H7V5H9M5,3H3V5H5M7,21H9V19H7M3,17H5V15H3M5,7H3V9H5M3,21H5V19H3V21Z" />
    </svg>
  }
}
