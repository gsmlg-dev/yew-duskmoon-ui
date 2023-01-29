#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FilePercentOutline)]
pub fn r#icon_file_percent_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 2H6C4.89 2 4 2.9 4 4V20C4 21.11 4.89 22 6 22H18C19.11 22 20 21.11 20 20V8L14 2M18 20H6V4H13V9H18V20M10 11.5C10 12.33 9.33 13 8.5 13S7 12.33 7 11.5 7.67 10 8.5 10 10 10.67 10 11.5M16 17.5C16 18.33 15.33 19 14.5 19S13 18.33 13 17.5 13.67 16 14.5 16 16 16.67 16 17.5M16 11.35L8.37 19L7 17.65L14.65 10L16 11.35Z" />
    </svg>
  }
}
