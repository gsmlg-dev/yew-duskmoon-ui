#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_TagMinusOutline)]
pub fn r#icon_tag_minus_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6.5 5C5.67 5 5 5.67 5 6.5S5.67 8 6.5 8 8 7.33 8 6.5 7.33 5 6.5 5M6.5 5C5.67 5 5 5.67 5 6.5S5.67 8 6.5 8 8 7.33 8 6.5 7.33 5 6.5 5M21.41 11.58L12.41 2.58C12.04 2.21 11.53 2 11 2H4C2.9 2 2 2.9 2 4V11C2 11.53 2.21 12.04 2.59 12.42L3 12.82C3.64 12.44 4.35 12.19 5.08 12.08L4 11V4H11L20 13L13 20L11.92 18.92C11.82 19.66 11.56 20.36 11.18 21L11.59 21.41C11.96 21.79 12.47 22 13 22C13.53 22 14.04 21.79 14.41 21.41L21.41 14.41C21.79 14.04 22 13.53 22 13C22 12.47 21.79 11.96 21.41 11.58M6.5 5C5.67 5 5 5.67 5 6.5S5.67 8 6.5 8 8 7.33 8 6.5 7.33 5 6.5 5M10 19H2V17H10V19Z" />
    </svg>
  }
}