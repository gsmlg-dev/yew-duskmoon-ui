#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_LessThan)]
pub fn r#icon_less_than(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.5,4.14L19.5,5.86L8.97,12L19.5,18.14L18.5,19.86L5,12L18.5,4.14Z" />
    </svg>
  }
}
