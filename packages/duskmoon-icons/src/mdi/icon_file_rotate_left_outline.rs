#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FileRotateLeftOutline)]
pub fn r#icon_file_rotate_left_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4 11C4 6.58 7.58 3 12 3L13 3.06V5.08L12 5C8.69 5 6 7.69 6 11H9L5 15L1 11H4M17 7H13C11.9 7 11 7.9 11 9V18C11 19.11 11.9 20 13 20H19C20.11 20 21 19.11 21 18V11L17 7M19 18H13V9H16V12H19V18Z" />
    </svg>
  }
}
