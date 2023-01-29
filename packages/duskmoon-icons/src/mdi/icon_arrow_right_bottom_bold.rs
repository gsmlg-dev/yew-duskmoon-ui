#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowRightBottomBold)]
pub fn r#icon_arrow_right_bottom_bold(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 3H7V10.5C7 12.43 8.57 14 10.5 14H13V10L20 16L13 22V18H10.5C6.36 18 3 14.64 3 10.5V3Z" />
    </svg>
  }
}
