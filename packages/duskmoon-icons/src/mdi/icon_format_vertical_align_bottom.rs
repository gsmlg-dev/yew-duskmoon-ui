#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FormatVerticalAlignBottom)]
pub fn r#icon_format_vertical_align_bottom(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16,13H13V3H11V13H8L12,17L16,13M4,19V21H20V19H4Z" />
    </svg>
  }
}
