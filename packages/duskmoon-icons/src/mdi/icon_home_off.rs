#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_HomeOff)]
pub fn r#icon_home_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.84 22.73L18.11 20H14V15.89L12.11 14H10V20H5V12H2L6.27 8.16L1.11 3L2.39 1.73L22.11 21.46L20.84 22.73M19 12H22L12 3L8.95 5.75L19 15.8V12Z" />
    </svg>
  }
}
