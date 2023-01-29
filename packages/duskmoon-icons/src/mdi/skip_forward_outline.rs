#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_SkipForwardOutline)]
pub fn r#icon_skip_forward_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6,9.83L8.17,12L6,14.17V9.83M4,5V19L11,12M20,5H18V19H20M13,9.83L15.17,12L13,14.17V9.83M11,5V19L18,12" />
    </svg>
  }
}
