#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Spray)]
pub fn r#icon_spray(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,4H12V6H10V4M7,3H9V5H7V3M7,6H9V8H7V6M6,8V10H4V8H6M6,5V7H4V5H6M6,2V4H4V2H6M13,22A2,2 0 0,1 11,20V10A2,2 0 0,1 13,8V7H14V4H17V7H18V8A2,2 0 0,1 20,10V20A2,2 0 0,1 18,22H13M13,10V20H18V10H13Z" />
    </svg>
  }
}
