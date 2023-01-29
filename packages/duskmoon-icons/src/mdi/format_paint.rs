#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FormatPaint)]
pub fn r#icon_format_paint(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18,4V3A1,1 0 0,0 17,2H5A1,1 0 0,0 4,3V7A1,1 0 0,0 5,8H17A1,1 0 0,0 18,7V6H19V10H9V21A1,1 0 0,0 10,22H12A1,1 0 0,0 13,21V12H21V4H18Z" />
    </svg>
  }
}
