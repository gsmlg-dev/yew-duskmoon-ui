#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Numeric10Circle)]
pub fn r#icon_numeric_10_circle(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 9H16V15H14V9M22 12C22 17.5 17.5 22 12 22S2 17.5 2 12 6.5 2 12 2 22 6.5 22 12M10 7H6V9H8V17H10V7M18 9C18 7.9 17.11 7 16 7H14C12.9 7 12 7.9 12 9V15C12 16.11 12.9 17 14 17H16C17.11 17 18 16.11 18 15V9Z" />
    </svg>
  }
}
