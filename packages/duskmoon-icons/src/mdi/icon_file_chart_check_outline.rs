#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FileChartCheckOutline)]
pub fn r#icon_file_chart_check_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M23.5 17L18.5 22L15 18.5L16.5 17L18.5 19L22 15.5L23.5 17M6 2C4.9 2 4 2.9 4 4V20C4 21.1 4.9 22 6 22H13.8C13.4 21.4 13.2 20.7 13.1 20H6V4H13V9H18V13.1C18.3 13 18.7 13 19 13C19.3 13 19.7 13 20 13.1V8L14 2H6M11 11V19H13V11H11M7 13V19H9V13H7Z" />
    </svg>
  }
}
