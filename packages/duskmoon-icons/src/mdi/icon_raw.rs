#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Raw)]
pub fn r#icon_raw(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6.5 9C7.3 9 8 9.7 8 10.5V11.5C8 12.1 7.6 12.6 7.1 12.9L8 15H6.5L5.6 13H4.5V15H3V9H6.5M6.5 11.5V10.5H4.5V11.5H6.5M10.25 9H12.75L14.25 15H12.75L12.38 13.5H10.63L10.25 15H8.75L10.25 9M11 12H12L11.75 11H11.25L11 12M20 9H21.5L20 15H18.5L17.74 11.96L17 15H15.5L14 9H15.5L16.24 12L17 9H18.5L19.24 12L20 9Z" />
    </svg>
  }
}
