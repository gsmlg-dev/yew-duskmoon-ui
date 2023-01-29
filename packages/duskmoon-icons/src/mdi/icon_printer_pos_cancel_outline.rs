#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PrinterPosCancelOutline)]
pub fn r#icon_printer_pos_cancel_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.5 14C16 14 14 16 14 18.5S16 23 18.5 23 23 21 23 18.5 21 14 18.5 14M18.5 21.5C16.84 21.5 15.5 20.16 15.5 18.5C15.5 17.94 15.65 17.42 15.92 17L20 21.08C19.58 21.35 19.06 21.5 18.5 21.5M21.08 20L17 15.92C17.42 15.65 17.94 15.5 18.5 15.5C20.16 15.5 21.5 16.84 21.5 18.5C21.5 19.06 21.35 19.58 21.08 20M7 15V13H11V15H7M6 17V12H20C20 10.9 19.11 10 18 10H17V4H7V10H6C4.89 10 4 10.9 4 12V19H12.03C12 18.83 12 18.67 12 18.5C12 18 12.07 17.5 12.18 17H6M9 6H15V10H9V6Z" />
    </svg>
  }
}
