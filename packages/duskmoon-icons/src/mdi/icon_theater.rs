#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Theater)]
pub fn r#icon_theater(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4,15H6A2,2 0 0,1 8,17V19H9V17A2,2 0 0,1 11,15H13A2,2 0 0,1 15,17V19H16V17A2,2 0 0,1 18,15H20A2,2 0 0,1 22,17V19H23V22H1V19H2V17A2,2 0 0,1 4,15M11,7L15,10L11,13V7M4,2H20A2,2 0 0,1 22,4V13.54C21.41,13.19 20.73,13 20,13V4H4V13C3.27,13 2.59,13.19 2,13.54V4A2,2 0 0,1 4,2Z" />
    </svg>
  }
}
