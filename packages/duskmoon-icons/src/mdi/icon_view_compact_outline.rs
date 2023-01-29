#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ViewCompactOutline)]
pub fn r#icon_view_compact_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3,5V19H22V5H3M5,7H20V11H5V7M5,17V13H9V17H5M11,17V13H20V17H11Z" />
    </svg>
  }
}
