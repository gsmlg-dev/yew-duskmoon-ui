#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PrinterWireless)]
pub fn r#icon_printer_wireless(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4.93,3.92L6.34,5.33C9.46,2.2 14.53,2.2 17.66,5.33L19.07,3.92C15.17,0 8.84,0 4.93,3.92M7.76,6.75L9.17,8.16C10.73,6.6 13.26,6.6 14.83,8.16L16.24,6.75C13.9,4.41 10.1,4.41 7.76,6.75M19,14A1,1 0 0,1 18,13A1,1 0 0,1 19,12A1,1 0 0,1 20,13A1,1 0 0,1 19,14M16,20H8V15H16V20M19,10H5A3,3 0 0,0 2,13V18H6V22H18V18H22V13A3,3 0 0,0 19,10Z" />
    </svg>
  }
}
