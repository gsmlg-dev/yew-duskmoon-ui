#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TramSide)]
pub fn r#icon_tram_side(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 1L5 4L7.5 6H5C5 6 2 6 2 9V19H7C7 19 7 17 9 17H22V14H18V8H22V6H10.5L13 4L9 1M4 8H9V14H4V8M11 8H16V14H11V8M4 16H5V18H4V16M9 19V19.5C9 20.88 10.12 22 11.5 22C12.5 22 13.39 21.41 13.79 20.5H15.21C15.61 21.41 16.5 22 17.5 22C18.88 22 20 20.88 20 19.5V19H9Z" />
    </svg>
  }
}
