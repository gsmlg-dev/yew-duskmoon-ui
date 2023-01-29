#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowTopLeftBottomRightBold)]
pub fn r#icon_arrow_top_left_bottom_right_bold(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5.83,8.66L3,11.5V3H11.5L8.66,5.83L18.17,15.34L21,12.5V21H12.5L15.34,18.17L5.83,8.66Z" />
    </svg>
  }
}
