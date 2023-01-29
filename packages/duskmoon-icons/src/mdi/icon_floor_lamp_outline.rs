#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FloorLampOutline)]
pub fn r#icon_floor_lamp_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 2L17 9H7L9 2M13.6 4H10.4L9.55 7H14.45M11 10H13V20H16V22H8V20H11Z" />
    </svg>
  }
}
