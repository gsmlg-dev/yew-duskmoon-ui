#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PrinterPosCogOutline)]
pub fn r#icon_printer_pos_cog_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.7 19.6V18.6L23.8 17.8C23.9 17.7 24 17.6 23.9 17.5L22.9 15.8C22.9 15.7 22.7 15.7 22.6 15.7L21.4 16.2C21.1 16 20.8 15.8 20.5 15.7L20.3 14.4C20.3 14.3 20.2 14.2 20.1 14.2H18.1C17.9 14.2 17.8 14.3 17.8 14.4L17.6 15.7C17.3 15.9 17.1 16 16.8 16.2L15.6 15.7C15.5 15.7 15.4 15.7 15.3 15.8L14.3 17.5C14.3 17.6 14.3 17.7 14.4 17.8L15.5 18.6V19.6L14.4 20.4C14.3 20.5 14.2 20.6 14.3 20.7L15.3 22.4C15.4 22.5 15.5 22.5 15.6 22.5L16.8 22C17 22.2 17.3 22.4 17.6 22.5L17.8 23.8C17.9 23.9 18 24 18.1 24H20.1C20.2 24 20.3 23.9 20.3 23.8L20.5 22.5C20.8 22.3 21 22.2 21.3 22L22.5 22.4C22.6 22.4 22.7 22.4 22.8 22.3L23.8 20.6C23.9 20.5 23.9 20.4 23.8 20.4L22.7 19.6M19 20.5C18.2 20.5 17.5 19.8 17.5 19S18.2 17.5 19 17.5 20.5 18.2 20.5 19 19.8 20.5 19 20.5M7 15V13H11V15H7M6 12H20C20 10.9 19.11 10 18 10H17V4H7V10H6C4.89 10 4 10.9 4 12V19H12C12 18.31 12.11 17.63 12.29 17H6V12M9 6H15V10H9V6Z" />
    </svg>
  }
}
