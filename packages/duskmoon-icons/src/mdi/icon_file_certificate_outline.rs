#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FileCertificateOutline)]
pub fn r#icon_file_certificate_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 13V11L12 12L10 11V13L8 14L10 15V17L12 16L14 17V15L16 14M14 2H7A2 2 0 0 0 5 4V18A2 2 0 0 0 7 20H8V18H7V4H13V8H17V18H16V20H17A2 2 0 0 0 19 18V7M14 13V11L12 12L10 11V13L8 14L10 15V17L12 16L14 17V15L16 14M10 23L12 22L14 23V18H10M14 13V11L12 12L10 11V13L8 14L10 15V17L12 16L14 17V15L16 14Z" />
    </svg>
  }
}
