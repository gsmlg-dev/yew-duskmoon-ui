#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Betamax)]
pub fn r#icon_betamax(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4,5A2,2 0 0,0 2,7V17A2,2 0 0,0 4,19H20A2,2 0 0,0 22,17V7A2,2 0 0,0 20,5H4M8,9A4,4 0 0,1 12,13A4,4 0 0,1 8,17A4,4 0 0,1 4,13A4,4 0 0,1 8,9M13,9H20V17H13V9M8,11A2,2 0 0,0 6,13A2,2 0 0,0 8,15A2,2 0 0,0 10,13A2,2 0 0,0 8,11Z" />
    </svg>
  }
}
