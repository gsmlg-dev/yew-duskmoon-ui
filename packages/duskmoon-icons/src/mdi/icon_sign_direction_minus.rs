#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SignDirectionMinus)]
pub fn r#icon_sign_direction_minus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.5 9.5L18 12H13V22H9A2 2 0 0 1 11 20V12H3.5L6 9.5L3.5 7H11V3L12 2L13 3V7H18M23 18H15V20H23Z" />
    </svg>
  }
}
