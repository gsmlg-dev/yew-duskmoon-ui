#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SkipNextOutline)]
pub fn r#icon_skip_next_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6,18L14.5,12L6,6M8,9.86L11.03,12L8,14.14M16,6H18V18H16" />
    </svg>
  }
}
