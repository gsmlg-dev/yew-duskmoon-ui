#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_InboxMultiple)]
pub fn r#icon_inbox_multiple(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19,8V5H5V8H9A3,3 0 0,0 12,11A3,3 0 0,0 15,8H19M19,3A2,2 0 0,1 21,5V12A2,2 0 0,1 19,14H5A2,2 0 0,1 3,12V5A2,2 0 0,1 5,3H19M3,15H9A3,3 0 0,0 12,18A3,3 0 0,0 15,15H21V19A2,2 0 0,1 19,21H5A2,2 0 0,1 3,19V15Z" />
    </svg>
  }
}
