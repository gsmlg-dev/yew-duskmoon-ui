#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowLeftThin)]
pub fn r#icon_arrow_left_thin(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10.05 16.94V12.94H18.97L19 10.93H10.05V6.94L5.05 11.94Z" />
    </svg>
  }
}
