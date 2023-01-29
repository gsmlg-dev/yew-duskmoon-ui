#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArchiveSettings)]
pub fn r#icon_archive_settings(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 2V6H3V2H21M4 7H20V20H4V7M9 12H15V10.5C15 10.22 14.78 10 14.5 10H9.5C9.22 10 9 10.22 9 10.5V12M7 24H9V22H7V24M11 24H13V22H11V24M15 24H17V22H15V24Z" />
    </svg>
  }
}
