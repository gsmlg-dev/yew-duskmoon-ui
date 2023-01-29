#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArchiveAlert)]
pub fn r#icon_archive_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2 4H18V7H2V4M3 8H17V20H3V8M7.5 11C7.22 11 7 11.22 7 11.5V13H13V11.5C13 11.22 12.78 11 12.5 11H7.5M20 13V7H22V13H20M20 17V15H22V17H20Z" />
    </svg>
  }
}
