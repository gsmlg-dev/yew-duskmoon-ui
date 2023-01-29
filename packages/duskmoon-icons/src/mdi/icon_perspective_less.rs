#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PerspectiveLess)]
pub fn r#icon_perspective_less(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.56,12L19.23,20H4.78L7.44,12H16.56M7,1L3,5L7,9V6H11V4H7V1M17,1V4H13V6H17V9L21,5L17,1M18,10H6L2,22H22L18,10Z" />
    </svg>
  }
}
