#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PrinterPosNetworkOutline)]
pub fn r#icon_printer_pos_network_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 11H17V13H13V11M22 20V22H15C15 22.55 14.55 23 14 23H10C9.45 23 9 22.55 9 22H2V20H9C9 19.45 9.45 19 10 19H11V17H4V10C4 8.9 4.89 8 6 8H7V2H17V8H18C19.11 8 20 8.9 20 10V17H13V19H14C14.55 19 15 19.45 15 20H22M9 8H15V4H9V8M18 15V10H6V15H18Z" />
    </svg>
  }
}
