#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_LabelOff)]
pub fn r#icon_label_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2,4.27L3.28,3L20,19.72L18.73,21L16.63,18.9C16.43,18.96 16.22,19 16,19H5A2,2 0 0,1 3,17V7C3,6.5 3.17,6.07 3.46,5.73L2,4.27M17.63,5.84L22,12L19,16.2L7.83,5H16C16.67,5 17.27,5.33 17.63,5.84Z" />
    </svg>
  }
}
