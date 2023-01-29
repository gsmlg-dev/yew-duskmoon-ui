#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_PhoneCheck)]
pub fn r#icon_phone_check(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6.62 10.79A15.1 15.1 0 0 0 13.21 17.38L15.41 15.18A1 1 0 0 1 16.41 14.93A11.36 11.36 0 0 0 20 15.5A1 1 0 0 1 21 16.5V20A1 1 0 0 1 20 21A17 17 0 0 1 3 4A1 1 0 0 1 4 3H7.5A1 1 0 0 1 8.5 4A11.36 11.36 0 0 0 9.07 7.57A1 1 0 0 1 8.82 8.57M21.71 4.71L20.29 3.29L15 8.59L12.71 6.29L11.29 7.71L15 11.41Z" />
    </svg>
  }
}
