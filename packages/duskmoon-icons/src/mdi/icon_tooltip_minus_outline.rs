#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TooltipMinusOutline)]
pub fn r#icon_tooltip_minus_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 2H4C2.9 2 2 2.9 2 4V16C2 17.11 2.9 18 4 18H8L12 22L16 18H20C21.11 18 22 17.11 22 16V4C22 2.9 21.11 2 20 2M20 16H15.17L12 19.17L8.83 16H4V4H20V16M16 11H8V9H16V11Z" />
    </svg>
  }
}
