#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FormatPilcrow)]
pub fn r#icon_format_pilcrow(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,11A4,4 0 0,1 6,7A4,4 0 0,1 10,3H18V5H16V21H14V5H12V21H10V11Z" />
    </svg>
  }
}
