#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_WhiteBalanceAuto)]
pub fn r#icon_white_balance_auto(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10.3,16L9.6,14H6.4L5.7,16H3.8L7,7H9L12.2,16M22,7L20.8,13.29L19.3,7H17.7L16.21,13.29L15,7H14.24C12.77,5.17 10.5,4 8,4A8,8 0 0,0 0,12A8,8 0 0,0 8,20C11.13,20 13.84,18.19 15.15,15.57L15.25,16H17L18.5,9.9L20,16H21.75L23.8,7M6.85,12.65H9.15L8,9L6.85,12.65Z" />
    </svg>
  }
}
