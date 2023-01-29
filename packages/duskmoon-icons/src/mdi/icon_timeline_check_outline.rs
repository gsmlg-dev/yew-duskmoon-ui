#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TimelineCheckOutline)]
pub fn r#icon_timeline_check_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 4H10C8.9 4 8 4.89 8 6V10L6 12L8 14V18C8 19.11 8.9 20 10 20H22C23.11 20 24 19.11 24 18V6C24 4.89 23.11 4 22 4M22 18H10V6H22V18M4 8H2V2H4V8M2 16H4V22H2V16M5 12C5 13.11 4.11 14 3 14C1.9 14 1 13.11 1 12C1 10.9 1.9 10 3 10C4.11 10 5 10.9 5 12M15 15.08L12.25 12.08L13.41 10.92L15 12.5L18.59 8.92L19.75 10.33L15 15.08Z" />
    </svg>
  }
}
