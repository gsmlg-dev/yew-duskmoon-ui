#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MapMarkerRight)]
pub fn r#icon_map_marker_right(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 11.5C7.62 11.5 6.5 10.38 6.5 9S7.62 6.5 9 6.5 11.5 7.62 11.5 9 10.38 11.5 9 11.5M9 2C5.13 2 2 5.13 2 9C2 14.25 9 22 9 22S16 14.25 16 9C16 5.13 12.87 2 9 2M18 17L23 12L18 7V17Z" />
    </svg>
  }
}
