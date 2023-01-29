#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FormatPageSplit)]
pub fn r#icon_format_page_split(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 11V7H13V11H11M13 1V5H11V1H13M11 17V13H13V17H11M9 20H6V4H9V2H6C4.89 2 4 2.9 4 4V20C4 21.11 4.89 22 6 22H9V20M15 3V8H18V20H15V22H18C19.11 22 20 21.11 20 20V8L15 3M13 19H11V23H13V19Z" />
    </svg>
  }
}
