#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_HandExtended)]
pub fn r#icon_hand_extended(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 13C20.6 13 21.1 13.2 21.5 13.6C21.8 14 22 14.5 22 15L14 18L7 16V7H8.9L16.2 9.7C16.7 9.9 17 10.3 17 10.8C17 11.1 16.9 11.4 16.7 11.6S16.1 12 15.8 12H13L11.2 11.3L10.9 12.2L13 13H20M1 7H5V18H1V7Z" />
    </svg>
  }
}