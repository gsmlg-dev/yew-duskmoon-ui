#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PrinterPosPause)]
pub fn r#icon_printer_pos_pause(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17 9H7V4H17V9M20 13.09C19.67 13.04 19.34 13 19 13C15.69 13 13 15.69 13 19H4V12C4 10.9 4.89 10 6 10H18C19.11 10 20 10.9 20 12V13.09M10 12H6V14H10V12M16 22H18V16H16V22M20 16V22H22V16H20Z" />
    </svg>
  }
}
