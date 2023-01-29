#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PhoneCheckOutline)]
pub fn r#icon_phone_check_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 15.5A11.36 11.36 0 0 1 16.43 14.93A1 1 0 0 0 15.43 15.18L13.23 17.38A15.1 15.1 0 0 1 6.64 10.79L8.84 8.59A1 1 0 0 0 9.09 7.59A11.36 11.36 0 0 1 8.5 4A1 1 0 0 0 7.5 3H4A1 1 0 0 0 3 4A17 17 0 0 0 20 21A1 1 0 0 0 21 20V16.5A1 1 0 0 0 20 15.5M5 5H6.54A12.54 12.54 0 0 0 7 7.59L5.79 8.8A15 15 0 0 1 5 5M19 19A15 15 0 0 1 15.2 18.24L16.41 17A12.31 12.31 0 0 0 19 17.45M15 11.44L11.29 7.74L12.71 6.32L15 8.59L20.29 3.29L21.71 4.71Z" />
    </svg>
  }
}
