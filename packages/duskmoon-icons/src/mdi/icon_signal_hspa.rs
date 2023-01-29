#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SignalHspa)]
pub fn r#icon_signal_hspa(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10.5,10.5H13.5V4.5H16.5V19.5H13.5V13.5H10.5V19.5H7.5V4.5H10.5V10.5Z" />
    </svg>
  }
}
