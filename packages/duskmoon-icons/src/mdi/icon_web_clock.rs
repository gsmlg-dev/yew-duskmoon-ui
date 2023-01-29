#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_WebClock)]
pub fn r#icon_web_clock(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 12.5V16.5L18 18.5L18.75 17.25L16.5 15.75V12.5H15M22 12.39C22 12.26 22 12.13 22 12C22 6.5 17.5 2 12 2C6.47 2 2 6.5 2 12C2 17.5 6.5 22 12 22C12.13 22 12.24 22 12.37 21.97C13.43 22.62 14.67 23 16 23C19.86 23 23 19.86 23 16C23 14.68 22.62 13.44 22 12.39M19.76 10.11C19.7 10.07 19.65 10.04 19.59 10H19.74C19.75 10.03 19.75 10.07 19.76 10.11M18.92 8H15.97C15.65 6.75 15.19 5.55 14.59 4.44C16.43 5.07 17.96 6.34 18.92 8M12 4.03C12.83 5.23 13.5 6.57 13.91 8H10.09C10.5 6.57 11.17 5.23 12 4.03M9.66 10H12.41C11.16 10.75 10.15 11.88 9.57 13.24C9.53 12.83 9.5 12.42 9.5 12C9.5 11.32 9.56 10.65 9.66 10M9.4 4.44C8.8 5.55 8.35 6.75 8 8H5.08C6.03 6.34 7.57 5.06 9.4 4.44M4.26 14C4.1 13.36 4 12.69 4 12S4.1 10.64 4.26 10H7.64C7.56 10.66 7.5 11.32 7.5 12S7.56 13.34 7.64 14H4.26M5.08 16H8C8.35 17.25 8.8 18.45 9.4 19.56C7.57 18.93 6.03 17.65 5.08 16M16 21C13.24 21 11 18.76 11 16S13.24 11 16 11 21 13.24 21 16 18.76 21 16 21Z" />
    </svg>
  }
}
