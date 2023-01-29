#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Consolidate)]
pub fn r#icon_consolidate(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 9H20V4H22V9A2 2 0 0 1 20 11H18V13L15 10L18 7M14 10A2 2 0 1 0 12 12A2 2 0 0 0 14 10M2 11V16H4V11H6V13L9 10L6 7V9H4A2 2 0 0 0 2 11M15 16L12 13L9 16H11V18A2 2 0 0 0 13 20H18V18H13V16" />
    </svg>
  }
}
