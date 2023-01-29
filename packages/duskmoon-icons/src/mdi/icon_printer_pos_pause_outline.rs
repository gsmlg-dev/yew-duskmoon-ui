#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PrinterPosPauseOutline)]
pub fn r#icon_printer_pos_pause_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 12C20 10.9 19.11 10 18 10H17V4H7V10H6C4.89 10 4 10.9 4 12V19H13C13 18.3 13.13 17.63 13.35 17H6V12H18V13.09C18.33 13.04 18.66 13 19 13C19.34 13 19.67 13.04 20 13.09V12M15 10H9V6H15V10M7 15V13H11V15H7M16 16H18V22H16V16M22 16V22H20V16H22Z" />
    </svg>
  }
}
