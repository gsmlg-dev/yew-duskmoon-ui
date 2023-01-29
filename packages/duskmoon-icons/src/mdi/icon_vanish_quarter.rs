#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_VanishQuarter)]
pub fn r#icon_vanish_quarter(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 3H13V8H11V3M4.9 6.3L6.3 4.9L9.1 7.7L7.8 9.2L4.9 6.3M8 13H3V11H8V13" />
    </svg>
  }
}
