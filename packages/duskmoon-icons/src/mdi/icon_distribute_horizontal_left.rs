#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DistributeHorizontalLeft)]
pub fn r#icon_distribute_horizontal_left(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 7V17H16V22H14V2H16V7H21M5 2H3V22H5V19H10V5H5V2Z" />
    </svg>
  }
}
