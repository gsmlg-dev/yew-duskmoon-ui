#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArchiveArrowUp)]
pub fn r#icon_archive_arrow_up(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4 21H20V8H4M14 15V18H10V15H7L12 10L17 15M3 3H21V7H3" />
    </svg>
  }
}
