#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ShareAllOutline)]
pub fn r#icon_share_all_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 9.8V10.7L11.3 10.9C8.7 11.3 6.8 12.3 5.4 13.6C7.1 13.1 8.9 12.8 11 12.8H13V14.1L15.2 12L13 9.8M11 5L18 12L11 19V14.9C6 14.9 2.5 16.5 0 20C1 15 4 10 11 9M17 8V5L24 12L17 19V16L21 12" />
    </svg>
  }
}
