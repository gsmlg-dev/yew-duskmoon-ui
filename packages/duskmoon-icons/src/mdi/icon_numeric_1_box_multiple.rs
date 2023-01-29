#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Numeric1BoxMultiple)]
pub fn r#icon_numeric_1_box_multiple(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3,5V21H19V23H3A2,2 0 0,1 1,21V5H3M21,1A2,2 0 0,1 23,3V17A2,2 0 0,1 21,19H7A2,2 0 0,1 5,17V3A2,2 0 0,1 7,1H21M14,15H16V5H12V7H14V15Z" />
    </svg>
  }
}
