#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PrinterPosStarOutline)]
pub fn r#icon_printer_pos_star_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 12H18V13.09C18.33 13.04 18.66 13 19 13C19.34 13 19.67 13.04 20 13.09V12C20 10.9 19.11 10 18 10H17V4H7V10H6C4.89 10 4 10.9 4 12V19H13C13 18.3 13.13 17.63 13.35 17H6V12M9 6H15V10H9V6M7 15V13H11V15H7M20.8 19.77L21.45 22.58L19 21.09L16.5 22.58L17.18 19.77L15 17.89L17.87 17.64L19 15L20.11 17.64L23 17.89L20.8 19.77Z" />
    </svg>
  }
}
