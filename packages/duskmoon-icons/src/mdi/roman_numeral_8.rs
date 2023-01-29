#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_RomanNumeral8)]
pub fn r#icon_roman_numeral_8(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M1 7L3 17H5L7 7H5L4 12L3 7H1M12 7V9H11V15H12V17H8V15H9V9H8V7H12M17 7V9H16V15H17V17H13V15H14V9H13V7H17M22 7V9H21V15H22V17H18V15H19V9H18V7H22Z" />
    </svg>
  }
}
