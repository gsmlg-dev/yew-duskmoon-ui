#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CalendarAccount)]
pub fn r#icon_calendar_account(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 9C14 9 15 11.42 13.59 12.84C12.17 14.26 9.75 13.25 9.75 11.25C9.75 10 10.75 9 12 9M16.5 18H7.5V16.88C7.5 15.63 9.5 14.63 12 14.63S16.5 15.63 16.5 16.88M19 19H5V8H19M16 1V3H8V1H6V3H5C3.9 3 3 3.9 3 5V19C3 20.11 3.9 21 5 21H19C20.11 21 21 20.11 21 19V5C21 3.9 20.11 3 19 3H18V1H16Z" />
    </svg>
  }
}
