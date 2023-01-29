#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SunClockOutline)]
pub fn r#icon_sun_clock_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.1 9.32L19.65 6L15.5 5.64C16.1 6.15 16.63 6.78 17.05 7.5C17.37 8.05 17.58 8.63 17.73 9.22C17.18 9.08 16.6 9 16 9C15.86 9 15.72 9 15.58 9C14.81 7.25 13.06 6 11 6C8.24 6 6 8.24 6 11C6 13.06 7.25 14.81 9 15.58C9 15.72 9 15.86 9 16C9 19.87 12.13 23 16 23S23 19.87 23 16C23 12.87 20.94 10.21 18.1 9.32M8 11C8 9.35 9.35 8 11 8C12.08 8 13.03 8.58 13.56 9.45C11.66 10.15 10.15 11.66 9.45 13.56C8.58 13.03 8 12.08 8 11M16 21C13.24 21 11 18.76 11 16S13.24 11 16 11 21 13.24 21 16 18.76 21 16 21M16.5 16.25L19.36 17.94L18.61 19.16L15 17V12H16.5V16.25M11 4C10.16 4 9.35 4.15 8.61 4.42L11 1L13.39 4.42C12.65 4.15 11.84 4 11 4M4.95 14.5C5.37 15.24 5.91 15.86 6.5 16.37L2.36 16L4.12 12.23C4.26 13 4.53 13.78 4.95 14.5M4.11 9.79L2.34 6L6.5 5.65C5.9 6.16 5.36 6.78 4.94 7.5C4.5 8.24 4.25 9 4.11 9.79Z" />
    </svg>
  }
}
