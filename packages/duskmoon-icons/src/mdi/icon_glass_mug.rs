#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_GlassMug)]
pub fn r#icon_glass_mug(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,4V7H18V4H10M8,2H20L21,2V3L20,4V20L21,21V22H20L8,22H7V21L8,20V18.6L4.2,16.83C3.5,16.5 3,15.82 3,15V8A2,2 0 0,1 5,6H8V4L7,3V2H8M5,15L8,16.39V8H5V15Z" />
    </svg>
  }
}
