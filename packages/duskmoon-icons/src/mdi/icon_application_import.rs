#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ApplicationImport)]
pub fn r#icon_application_import(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M1 12H10.8L8.3 9.5L9.7 8.1L14.6 13L9.7 17.9L8.3 16.5L10.8 14H1V12M21 2H3C1.9 2 1 2.9 1 4V10.1H3V6H21V20H3V16H1V20C1 21.1 1.9 22 3 22H21C22.1 22 23 21.1 23 20V4C23 2.9 22.1 2 21 2" />
    </svg>
  }
}
