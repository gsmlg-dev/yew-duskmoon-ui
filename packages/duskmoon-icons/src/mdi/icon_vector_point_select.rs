#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_VectorPointSelect)]
pub fn r#icon_vector_point_select(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,20L7,22L12,11L17,22L12,20M8,2H16V5H22V7H16V10H8V7H2V5H8V2M10,4V8H14V4H10Z" />
    </svg>
  }
}
