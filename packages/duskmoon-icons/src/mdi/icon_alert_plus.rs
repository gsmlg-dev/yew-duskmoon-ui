#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AlertPlus)]
pub fn r#icon_alert_plus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 19C14 16.21 15.91 13.87 18.5 13.2L12 2L1 21H14.35C14.13 20.37 14 19.7 14 19M13 18H11V16H13V18M13 14H11V10H13V14M21 15V18H24V20H21V23H19V20H16V18H19V15H21Z" />
    </svg>
  }
}
