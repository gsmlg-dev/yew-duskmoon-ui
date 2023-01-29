#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_PrinterPosOutline)]
pub fn r#icon_printer_pos_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 10H17V4H7V10H6C4.89 10 4 10.9 4 12V19H20V12C20 10.9 19.11 10 18 10M9 6H15V10H9V6M18 17H6V12H18V17M17 15H13V13H17V15Z" />
    </svg>
  }
}
