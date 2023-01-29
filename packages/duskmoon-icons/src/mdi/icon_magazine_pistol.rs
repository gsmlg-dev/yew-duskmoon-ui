#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MagazinePistol)]
pub fn r#icon_magazine_pistol(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 1L12 3H7L9 21H8V23H18V21L16 1M9 5H12L12.24 7H9.24M9.47 9H12.47L12.71 11H9.71M9.94 13H12.94L13.18 15H10.18M10.41 17H13.41L13.65 19H10.65Z" />
    </svg>
  }
}
