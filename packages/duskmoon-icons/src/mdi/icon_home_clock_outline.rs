#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_HomeClockOutline)]
pub fn r#icon_home_clock_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.5 16.25L19.36 17.94L18.61 19.16L15 17V12H16.5V16.25M23 16C23 19.87 19.87 23 16 23C13.62 23 11.53 21.81 10.26 20H4V12H1L11 3L18 9.29C20.89 10.15 23 12.83 23 16M9.29 18C9.11 17.37 9 16.7 9 16C9 12.54 11.5 9.68 14.8 9.11L11 5.69L6 10.19V18H9.29M21 16C21 13.24 18.76 11 16 11S11 13.24 11 16 13.24 21 16 21 21 18.76 21 16Z" />
    </svg>
  }
}
