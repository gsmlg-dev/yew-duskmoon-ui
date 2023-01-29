#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_KeyboardCaps)]
pub fn r#icon_keyboard_caps(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6,18H18V16H6M12,8.41L16.59,13L18,11.58L12,5.58L6,11.58L7.41,13L12,8.41Z" />
    </svg>
  }
}
