#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SizeXs)]
pub fn r#icon_size_xs(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 7H8L9 9.5L10 7H12L10 12L12 17H10L9 14.5L8 17H6L8 12L6 7M15 7H19V9H15V11H17C18.11 11 19 11.9 19 13V15C19 16.11 18.11 17 17 17H13V15H17V13H15C13.9 13 13 12.11 13 11V9C13 7.9 13.9 7 15 7Z" />
    </svg>
  }
}
