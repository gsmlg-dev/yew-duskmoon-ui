#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ChevronTripleRight)]
pub fn r#icon_chevron_triple_right(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.58,16.59L19.17,12L14.58,7.41L16,6L22,12L16,18L14.58,16.59M8.58,16.59L13.17,12L8.58,7.41L10,6L16,12L10,18L8.58,16.59M2.58,16.59L7.17,12L2.58,7.41L4,6L10,12L4,18L2.58,16.59Z" />
    </svg>
  }
}
