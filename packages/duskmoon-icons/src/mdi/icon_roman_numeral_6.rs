#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RomanNumeral6)]
pub fn r#icon_roman_numeral_6(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 7L8 17H10L12 7H10L9 12L8 7H6M17 7V9H16V15H17V17H13V15H14V9H13V7H17Z" />
    </svg>
  }
}
