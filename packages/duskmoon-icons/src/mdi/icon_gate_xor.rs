#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_GateXor)]
pub fn r#icon_gate_xor(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2,4C5,10 5,14 2,20H4C7,14 7,10 4.1,4H2M6,4C9,10 9,14 6,20H9C14,20 18,17 22,12C18,7 14,4 9,4H6M9,6C12.8,6 16,8.1 19.3,12C15.9,15.9 12.8,18 9,18C10.5,14 10.5,10 9,6Z" />
    </svg>
  }
}
