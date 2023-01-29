#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_KeyboardF7)]
pub fn r#icon_keyboard_f7(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 7H11V9H7V11H10V13H7V17H5V7M15 17H13L17 9H13V7H19V9L15 17Z" />
    </svg>
  }
}
