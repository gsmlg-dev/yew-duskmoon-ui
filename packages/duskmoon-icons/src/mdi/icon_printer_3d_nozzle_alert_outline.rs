#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Printer3dNozzleAlertOutline)]
pub fn r#icon_printer_3d_nozzle_alert_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7 2H17V7H19V13H16.5L13 17H11L7.5 13H5V7H7V2M10 22H2V20H10C10.6 20 11 19.5 11 19V18H13V19C13 20.7 11.7 22 10 22M7 9V11H8.5L12 15L15.5 11H17V9H15V4H9V9H7M21 13V7H23V13H21M21 17V15H23V17H21Z" />
    </svg>
  }
}
