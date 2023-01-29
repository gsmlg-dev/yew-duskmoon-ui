#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PrinterPosOffOutline)]
pub fn r#icon_printer_pos_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2.39 1.73L1.11 3L7 8.89V10H6C4.89 10 4 10.9 4 12V19H17.11L20.84 22.73L22.11 21.46L2.39 1.73M6 17V12H10.11L15.11 17H6M9.2 6L7.2 4H17V10H18C19.11 10 20 10.9 20 12V16.8L18 14.8V12H15.2L13.2 10H15V6H9.2M7 13H11V15H7V13Z" />
    </svg>
  }
}
