#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AppleKeyboardShift)]
pub fn r#icon_apple_keyboard_shift(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15,18V12H17.17L12,6.83L6.83,12H9V18H15M12,4L22,14H17V20H7V14H2L12,4Z" />
    </svg>
  }
}
