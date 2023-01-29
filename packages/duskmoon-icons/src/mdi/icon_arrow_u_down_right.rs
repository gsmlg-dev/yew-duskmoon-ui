#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowUDownRight)]
pub fn r#icon_arrow_u_down_right(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.5 14.5L16 20L10.5 14.5L11.91 13.09L15 16.17V10.5C15 8 13 6 10.5 6S6 8 6 10.5V18H4V10.5C4 6.91 6.91 4 10.5 4S17 6.91 17 10.5V16.17L20.09 13.08L21.5 14.5Z" />
    </svg>
  }
}
