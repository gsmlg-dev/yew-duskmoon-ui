#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_EmailAlert)]
pub fn r#icon_email_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 8L10 13L2 8V6L10 11L18 6M18 4H2C.9 4 0 4.9 0 6V18C0 19.1 .9 20 2 20H18C19.1 20 20 19.1 20 18V6C20 4.9 19.1 4 18 4M24 7H22V13H24V7M24 15H22V17H24V15Z" />
    </svg>
  }
}
