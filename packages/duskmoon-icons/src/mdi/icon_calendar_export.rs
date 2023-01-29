#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CalendarExport)]
pub fn r#icon_calendar_export(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 22L16 18H13V12H11V18H8L12 22M19 3H18V1H16V3H8V1H6V3H5C3.9 3 3 3.9 3 5V19C3 20.11 3.9 21 5 21H8L6 19H5V8H19V19H18L16 21H19C20.11 21 21 20.11 21 19V5C21 3.9 20.11 3 19 3Z" />
    </svg>
  }
}
