#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PrinterPosOff)]
pub fn r#icon_printer_pos_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.2 9L7.2 4H17V9H12.2M20 16.8V12C20 10.9 19.11 10 18 10H13.2L20 16.8M22.11 21.46L20.84 22.73L17.11 19H4V12C4 10.9 4.89 10 6 10H8.11L1.11 3L2.39 1.73L22.11 21.46M10 12H6V14H10V12Z" />
    </svg>
  }
}
