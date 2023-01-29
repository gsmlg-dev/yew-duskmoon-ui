#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ReceiptTextMinusOutline)]
pub fn r#icon_receipt_text_minus_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 18V20H23V18H15M13.26 20.74L12 22L10.5 20.5L9 22L7.5 20.5L6 22L4.5 20.5L3 22V2L4.5 3.5L6 2L7.5 3.5L9 2L10.5 3.5L12 2L13.5 3.5L15 2L16.5 3.5L18 2L19.5 3.5L21 2V13.35C20.37 13.13 19.7 13 19 13V5H5V19H13C13 19.57 13.1 20.22 13.26 20.74M14.54 15C14 15.58 13.61 16.25 13.35 17H6V15H14.54M6 11H18V13H6V11M6 7H18V9H6V7Z" />
    </svg>
  }
}