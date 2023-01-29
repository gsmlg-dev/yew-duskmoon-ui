#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Numeric0BoxMultipleOutline)]
pub fn r#icon_numeric_0_box_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21,17V3H7V17H21M21,1A2,2 0 0,1 23,3V17A2,2 0 0,1 21,19H7A2,2 0 0,1 5,17V3A2,2 0 0,1 7,1H21M3,5V21H19V23H3A2,2 0 0,1 1,21V5H3M13,5H15A2,2 0 0,1 17,7V13A2,2 0 0,1 15,15H13A2,2 0 0,1 11,13V7A2,2 0 0,1 13,5M13,7V13H15V7H13Z" />
    </svg>
  }
}
