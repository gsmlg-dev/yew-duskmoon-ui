#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ApplicationSettings)]
pub fn r#icon_application_settings(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 0H3C1.9 0 1 .9 1 2V18C1 19.1 1.9 20 3 20H21C22.1 20 23 19.1 23 18V2C23 .9 22.1 0 21 0M21 5H3V2H21V5M7 22H9V24H7V22M11 22H13V24H11V22M15 22H17V24H15V22" />
    </svg>
  }
}
