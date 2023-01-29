#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FormatHorizontalAlignRight)]
pub fn r#icon_format_horizontal_align_right(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13,8V11H3V13H13V16L17,12L13,8M19,20H21V4H19V20Z" />
    </svg>
  }
}
