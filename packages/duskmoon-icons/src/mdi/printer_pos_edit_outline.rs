#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_PrinterPosEditOutline)]
pub fn r#icon_printer_pos_edit_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 17V12H18.13L19.39 10.74C19.42 10.71 19.45 10.69 19.5 10.67C19.11 10.26 18.59 10 18 10H17V4H7V10H6C4.89 10 4 10.9 4 12V19H11.13L13.13 17H6M9 6H15V10H9V6M7 15V13H11V15H7M22.85 14.19L21.87 15.17L19.83 13.13L20.81 12.15C21 11.95 21.33 11.95 21.53 12.15L22.85 13.47C23.05 13.67 23.05 14 22.85 14.19M19.13 13.83L21.17 15.87L15.04 22H13V19.96L19.13 13.83Z" />
    </svg>
  }
}