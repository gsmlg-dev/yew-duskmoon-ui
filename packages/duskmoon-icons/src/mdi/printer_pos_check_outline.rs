#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_PrinterPosCheckOutline)]
pub fn r#icon_printer_pos_check_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7 15V13H11V15H7M6 17H13.35C13.13 17.63 13 18.3 13 19H4V12C4 10.9 4.89 10 6 10H7V4H17V10H18C19.11 10 20 10.9 20 12V13.09C19.67 13.04 19.34 13 19 13C18.66 13 18.33 13.04 18 13.09V12H6V17M9 10H15V6H9V10M21.34 15.84L17.75 19.43L16.16 17.84L15 19L17.75 22L22.5 17.25L21.34 15.84Z" />
    </svg>
  }
}
