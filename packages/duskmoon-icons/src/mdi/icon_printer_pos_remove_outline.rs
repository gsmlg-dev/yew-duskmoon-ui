#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PrinterPosRemoveOutline)]
pub fn r#icon_printer_pos_remove_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7 15V13H11V15H7M6 17H13.35C13.13 17.63 13 18.3 13 19H4V12C4 10.9 4.89 10 6 10H7V4H17V10H18C19.11 10 20 10.9 20 12V13.09C19.67 13.04 19.34 13 19 13C18.66 13 18.33 13.04 18 13.09V12H6V17M9 10H15V6H9V10M22.54 16.88L21.12 15.47L19 17.59L16.88 15.47L15.47 16.88L17.59 19L15.47 21.12L16.88 22.54L19 20.41L21.12 22.54L22.54 21.12L20.41 19L22.54 16.88Z" />
    </svg>
  }
}
