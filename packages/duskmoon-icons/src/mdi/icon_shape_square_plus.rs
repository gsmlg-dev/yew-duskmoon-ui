#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ShapeSquarePlus)]
pub fn r#icon_shape_square_plus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19,5H22V7H19V10H17V7H14V5H17V2H19V5M17,19V13H19V21H3V5H11V7H5V19H17Z" />
    </svg>
  }
}
