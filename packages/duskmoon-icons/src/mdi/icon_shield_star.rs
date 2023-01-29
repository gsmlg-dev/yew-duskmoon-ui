#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ShieldStar)]
pub fn r#icon_shield_star(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 1L3 5V11C3 16.55 6.84 21.74 12 23C17.16 21.74 21 16.55 21 11V5L12 1M15.08 16L12 14.15L8.93 16L9.74 12.5L7.03 10.16L10.61 9.85L12 6.55L13.39 9.84L16.97 10.15L14.26 12.5L15.08 16Z" />
    </svg>
  }
}
