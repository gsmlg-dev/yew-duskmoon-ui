#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_MessageFlash)]
pub fn r#icon_message_flash(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 4C22 2.9 21.1 2 20 2H4C2.9 2 2 2.9 2 4V22L6 18H15V10H22V4M22.5 16H20.3L22 12H17V18H19V23L22.5 16Z" />
    </svg>
  }
}
