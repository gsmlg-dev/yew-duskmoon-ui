#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FolderStarMultipleOutline)]
pub fn r#icon_folder_star_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2 6V20H20V22H2C.895 22 0 21.11 0 20V6H2M12.78 10.05L15.81 9.79L17 7L18.19 9.79L21.22 10.05L18.92 12.04L19.61 15L17 13.47L14.39 15L15.08 12.04L12.78 10.05M24 6V16C24 17.11 23.11 18 22 18H6C4.9 18 4 17.11 4 16V4C4 2.89 4.9 2 6 2H12L14 4H22C23.11 4 24 4.9 24 6M22 6H6V16H22V6Z" />
    </svg>
  }
}
