#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TestTubeEmpty)]
pub fn r#icon_test_tube_empty(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7,2H17V4H16V18A4,4 0 0,1 12,22A4,4 0 0,1 8,18V4H7V2M14,4H10V18A2,2 0 0,0 12,20A2,2 0 0,0 14,18V4Z" />
    </svg>
  }
}
