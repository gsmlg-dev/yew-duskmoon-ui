#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_KeyboardTabReverse)]
pub fn r#icon_keyboard_tab_reverse(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4 6H2V18H4M11 6L5 12L11 18L12.41 16.58L8.83 13H23V11H8.83L12.41 7.41L11 6Z" />
    </svg>
  }
}
