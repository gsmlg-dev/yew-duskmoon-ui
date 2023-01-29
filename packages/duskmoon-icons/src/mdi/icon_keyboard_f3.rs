#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_KeyboardF3)]
pub fn r#icon_keyboard_f3(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 7H11V9H7V11H10V13H7V17H5V7M19 15C19 16.11 18.11 17 17 17H13V15H17V13H15V11H17V9H13V7H17C18.1 7 19 7.89 19 9V10.5C19 11.33 18.33 12 17.5 12C18.33 12 19 12.67 19 13.5V15Z" />
    </svg>
  }
}
