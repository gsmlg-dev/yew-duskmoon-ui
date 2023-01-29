#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowDecisionOutline)]
pub fn r#icon_arrow_decision_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9.64,13.4C8.63,12.5 7.34,12.03 6,12V15L2,11L6,7V10C7.67,10 9.3,10.57 10.63,11.59C10.22,12.15 9.89,12.76 9.64,13.4M18,15V12C17.5,12 13.5,12.16 13.05,16.2C14.61,16.75 15.43,18.47 14.88,20.03C14.33,21.59 12.61,22.41 11.05,21.86C9.5,21.3 8.67,19.59 9.22,18.03C9.5,17.17 10.2,16.5 11.05,16.2C11.34,12.61 14.4,9.88 18,10V7L22,11L18,15M13,19A1,1 0 0,0 12,18A1,1 0 0,0 11,19A1,1 0 0,0 12,20A1,1 0 0,0 13,19M11,11.12C11.58,10.46 12.25,9.89 13,9.43V5H16L12,1L8,5H11V11.12Z" />
    </svg>
  }
}