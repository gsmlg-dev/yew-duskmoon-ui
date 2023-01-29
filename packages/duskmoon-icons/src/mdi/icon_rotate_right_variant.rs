#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RotateRightVariant)]
pub fn r#icon_rotate_right_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,4V1L14,5L10,9V6A6,6 0 0,0 4,12L4.08,13H2.06L2,12A8,8 0 0,1 10,4M17,2H20A2,2 0 0,1 22,4V20A2,2 0 0,1 20,22H17A2,2 0 0,1 15,20V4A2,2 0 0,1 17,2M4,15H13V22H4A2,2 0 0,1 2,20V17A2,2 0 0,1 4,15Z" />
    </svg>
  }
}
