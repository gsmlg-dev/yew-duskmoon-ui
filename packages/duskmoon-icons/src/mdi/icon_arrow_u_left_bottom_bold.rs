#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowULeftBottomBold)]
pub fn r#icon_arrow_u_left_bottom_bold(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 10.5C21 14.64 17.64 18 13.5 18H11V22L4 16L11 10V14H13.5C15.43 14 17 12.43 17 10.5S15.43 7 13.5 7H6V3H13.5C17.64 3 21 6.36 21 10.5Z" />
    </svg>
  }
}
