#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowTopRightThin)]
pub fn r#icon_arrow_top_right_thin(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11.93 5L14.76 7.83L5 17.59L6.42 19L16.18 9.25L19 12.07V5H11.93Z" />
    </svg>
  }
}
