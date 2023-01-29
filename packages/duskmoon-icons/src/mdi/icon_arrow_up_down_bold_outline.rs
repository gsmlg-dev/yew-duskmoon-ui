#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowUpDownBoldOutline)]
pub fn r#icon_arrow_up_down_bold_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16,10H22L12,0L2,10H8V14H2L12,24L22,14H16V10M14,16H17L12,21L7,16H10V8H7L12,3L17,8H14V16Z" />
    </svg>
  }
}
