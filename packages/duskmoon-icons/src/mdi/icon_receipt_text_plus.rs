#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ReceiptTextPlus)]
pub fn r#icon_receipt_text_plus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 15V18H15V20H18V23H20V20H23V18H20V15H18M19.5 3.5L18 2L16.5 3.5L15 2L13.5 3.5L12 2L10.5 3.5L9 2L7.5 3.5L6 2L4.5 3.5L3 2V22L4.5 20.5L6 22L7.5 20.5L9 22L10.5 20.5L12 22L13.26 20.74C13.09 20.18 13 19.59 13 19C13 18.32 13.12 17.64 13.34 17H6V15H14.53C15.67 13.73 17.29 13 19 13C19.68 13 20.36 13.12 21 13.34V2L19.5 3.5M18 13H6V11H18V13M18 9H6V7H18V9Z" />
    </svg>
  }
}
