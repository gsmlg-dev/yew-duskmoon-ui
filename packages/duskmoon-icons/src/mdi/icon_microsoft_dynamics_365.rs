#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MicrosoftDynamics365)]
pub fn r#icon_microsoft_dynamics_365(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6,2L17.25,8.5L13.5,11.5L6,8V2M6,9L9.5,11.25L6,22L18,9V15L6,22V9Z" />
    </svg>
  }
}
