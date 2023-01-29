#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FormatAlignMiddle)]
pub fn r#icon_format_align_middle(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13,6L15,4L16.42,5.42L12,9.84L7.58,5.42L9,4L11,6V2H13V6M3,11H21V13H3V11M13,18V22H11V18L9,20L7.58,18.58L12,14.16L16.42,18.58L15,20L13,18Z" />
    </svg>
  }
}
