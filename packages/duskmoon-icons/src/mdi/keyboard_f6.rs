#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_KeyboardF6)]
pub fn r#icon_keyboard_f6(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 7H11V9H7V11H10V13H7V17H5V7M15 7H19V9H15V11H17C18.11 11 19 11.9 19 13V15C19 16.11 18.11 17 17 17H15C13.9 17 13 16.11 13 15V9C13 7.9 13.9 7 15 7M15 13V15H17V13H15Z" />
    </svg>
  }
}
