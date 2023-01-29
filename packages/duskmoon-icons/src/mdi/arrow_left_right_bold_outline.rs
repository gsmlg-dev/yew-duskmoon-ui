#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ArrowLeftRightBoldOutline)]
pub fn r#icon_arrow_left_right_bold_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14,16V22L24,12L14,2V8H10V2L0,12L10,22V16H14M8,14V17L3,12L8,7V10H16V7L21,12L16,17V14H8Z" />
    </svg>
  }
}
