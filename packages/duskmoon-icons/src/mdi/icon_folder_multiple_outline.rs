#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FolderMultipleOutline)]
pub fn r#icon_folder_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22,4A2,2 0 0,1 24,6V16A2,2 0 0,1 22,18H6A2,2 0 0,1 4,16V4A2,2 0 0,1 6,2H12L14,4H22M2,6V20H20V22H2A2,2 0 0,1 0,20V11H0V6H2M6,6V16H22V6H6Z" />
    </svg>
  }
}
