#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SetSquare)]
pub fn r#icon_set_square(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17.7 17.7L16.6 18.8L15.9 18L17 17L15 15L14 16.1L13.3 15.4L14.4 14.3L12.5 12.4L11.4 13.5L10.7 12.8L11.8 11.7L9.8 9.8L8.7 10.9L8 10.2L9 9L7.1 7.1L6 8.1L5.3 7.4L6.4 6.3L4 4V20H20L17.7 17.7M7 17V11.2L12.8 17H7Z" />
    </svg>
  }
}
