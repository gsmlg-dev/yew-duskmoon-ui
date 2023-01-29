#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FormatHorizontalAlignLeft)]
pub fn r#icon_format_horizontal_align_left(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11,16V13H21V11H11V8L7,12L11,16M3,20H5V4H3V20Z" />
    </svg>
  }
}
