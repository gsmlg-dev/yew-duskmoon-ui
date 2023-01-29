#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Printer3dNozzleHeat)]
pub fn r#icon_printer_3d_nozzle_heat(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4 2H14V7H16V13H13.5L10 17H8L4.5 13H2V7H4V2M23 14.5L21.6 16.7L23 18.9L21 22L19.2 21.1L20.7 18.9L19.2 16.7L21.2 13.6L23 14.5M18.7 14.5L17.2 16.7L18.7 18.9L16.7 22L14.9 21.1L16.3 18.9L14.9 16.7L16.9 13.6L18.7 14.5" />
    </svg>
  }
}
