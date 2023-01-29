#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PotMixOutline)]
pub fn r#icon_pot_mix_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.6 9L18 3.1L19.7 4.1L16.9 9H14.6M16.3 10H21V12H19V19C19 20.1 18.1 21 17 21H7C5.9 21 5 20.1 5 19V12H3V10H16.3M17 12H7V19H17V12Z" />
    </svg>
  }
}
