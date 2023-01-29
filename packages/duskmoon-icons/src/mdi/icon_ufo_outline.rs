#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_UfoOutline)]
pub fn r#icon_ufo_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17 10.54C16.78 7.44 14.63 5 12 5S7.22 7.44 7 10.54C4 11.23 2 12.5 2 14C2 16.21 6.5 18 12 18S22 16.21 22 14C22 12.5 20 11.23 17 10.54M14.93 11.84C13.03 12.05 10.97 12.05 9.07 11.84C9.03 11.56 9 11.28 9 11C9 8.8 10.35 7 12 7S15 8.8 15 11C15 11.28 15 11.56 14.93 11.84Z" />
    </svg>
  }
}
