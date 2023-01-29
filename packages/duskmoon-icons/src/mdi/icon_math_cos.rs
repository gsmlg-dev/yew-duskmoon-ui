#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MathCos)]
pub fn r#icon_math_cos(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4,7A2,2 0 0,0 2,9V15A2,2 0 0,0 4,17H6A2,2 0 0,0 8,15V14H6V15H4V9H6V10H8V9A2,2 0 0,0 6,7H4M11,7A2,2 0 0,0 9,9V15A2,2 0 0,0 11,17H13A2,2 0 0,0 15,15V9A2,2 0 0,0 13,7H11M11,9H13V15H11V9M18,7A2,2 0 0,0 16,9V11A2,2 0 0,0 18,13H20V15H16V17H20A2,2 0 0,0 22,15V13A2,2 0 0,0 20,11H18V9H22V7H18Z" />
    </svg>
  }
}