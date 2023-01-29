#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_LightningBoltOutline)]
pub fn r#icon_lightning_bolt_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 9.47V11H14.76L13 14.53V13H9.24L11 9.47M13 1L6 15H11V23L18 9H13V1Z" />
    </svg>
  }
}
