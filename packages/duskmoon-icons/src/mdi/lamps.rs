#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Lamps)]
pub fn r#icon_lamps(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10 2L12 9H2L4 2M6 10H8V20H11V22H3V20H6V10M20 8L22 15H12L14 8M16 16H18V20H21V22H13V20H16V16Z" />
    </svg>
  }
}
