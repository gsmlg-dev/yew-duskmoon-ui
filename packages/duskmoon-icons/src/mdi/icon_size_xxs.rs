#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SizeXxs)]
pub fn r#icon_size_xxs(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 7H11L12 9.5L13 7H15L13 12L15 17H13L12 14.5L11 17H9L11 12L9 7M18 7H22V9H18V11H20C21.11 11 22 11.9 22 13V15C22 16.11 21.11 17 20 17H16V15H20V13H18C16.9 13 16 12.11 16 11V9C16 7.9 16.9 7 18 7M2 7H4L5 9.5L6 7H8L6 12L8 17H6L5 14.5L4 17H2L4 12L2 7Z" />
    </svg>
  }
}
