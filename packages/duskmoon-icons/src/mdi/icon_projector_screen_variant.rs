#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ProjectorScreenVariant)]
pub fn r#icon_projector_screen_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 18V9H20C20.55 9 21 8.55 21 8V7C21 6.45 20.55 6 20 6H4C3.45 6 3 6.45 3 7V8C3 8.55 3.45 9 4 9H5V18H19Z" />
    </svg>
  }
}
