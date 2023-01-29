#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ArrowTopLeftThick)]
pub fn r#icon_arrow_top_left_thick(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9.12,11.94V16.89H5.69V5.69H16.89V9.12H11.94L18.31,15.5L15.5,18.31L9.12,11.94Z" />
    </svg>
  }
}
