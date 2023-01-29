#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RomanNumeral4)]
pub fn r#icon_roman_numeral_4(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 7L14 17H16L18 7H16L15 12L14 7H12M11 7V9H10V15H11V17H7V15H8V9H7V7H11Z" />
    </svg>
  }
}
