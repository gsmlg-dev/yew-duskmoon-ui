#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ClockMinusOutline)]
pub fn r#icon_clock_minus_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.47 15.08L11 13V7H12.5V12.25L15.58 14.08C15.17 14.36 14.79 14.7 14.47 15.08M13.08 19.92C12.72 19.97 12.37 20 12 20C7.58 20 4 16.42 4 12S7.58 4 12 4 20 7.58 20 12C20 12.37 19.97 12.72 19.92 13.08C20.61 13.18 21.25 13.4 21.84 13.72C21.94 13.16 22 12.59 22 12C22 6.5 17.5 2 12 2S2 6.5 2 12C2 17.5 6.47 22 12 22C12.59 22 13.16 21.94 13.72 21.84C13.4 21.25 13.18 20.61 13.08 19.92M15 18V20H23V18H15Z" />
    </svg>
  }
}
