#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(BS_CcSquareFill)]
pub fn r#icon_cc_square_fill(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2H2Zm3.408 5.89c-.83 0-1.318.64-1.318 1.753v.742c0 1.108.479 1.727 1.318 1.727.69 0 1.138-.435 1.187-1.05h1.147v.114c-.058 1.147-1.029 1.938-2.343 1.938-1.612 0-2.518-1.028-2.518-2.729v-.747c0-1.7.914-2.75 2.518-2.75 1.319 0 2.29.812 2.343 1.999v.11H6.595c-.049-.638-.506-1.108-1.187-1.108Zm5.404 0c-.831 0-1.319.64-1.319 1.753v.742c0 1.108.48 1.727 1.319 1.727.69 0 1.138-.435 1.186-1.05h1.147v.114c-.057 1.147-1.028 1.938-2.342 1.938-1.613 0-2.518-1.028-2.518-2.729v-.747c0-1.7.914-2.75 2.518-2.75 1.318 0 2.29.812 2.342 1.999v.11h-1.147c-.048-.638-.505-1.108-1.186-1.108Z"/>
    </svg>
  }
}
