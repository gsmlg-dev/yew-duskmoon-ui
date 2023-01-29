#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_GamepadDown)]
pub fn r#icon_gamepad_down(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9,2V7.5L12,10.5L15,7.5V2H9M2,9V15H7.5L10.5,12L7.5,9H2M16.5,9L13.5,12L16.5,15H22V9H16.5M12,13.5L9,16.5V22H15V16.5L12,13.5M11,18H13V20H11V18Z" />
    </svg>
  }
}
