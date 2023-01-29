#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ThermostatBoxAuto)]
pub fn r#icon_thermostat_box_auto(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 14H18L14.8 23H16.7L17.4 21H20.6L21.3 23H23.2L20 14M17.8 19.7L19 16L20.2 19.7H17.8M15.4 15.4L14.8 14.8C15.5 14.1 16 13.1 16 12C16 11.3 15.8 10.6 15.4 10L17.6 7.8C18.5 9 19 10.4 19 12H21V5C21 3.9 20.1 3 19 3H5C3.9 3 3 3.9 3 5V19C3 20.1 3.9 21 5 21H13.4L15.4 15.4M9.2 14.8L7 17C5.7 15.7 5 14 5 12C5 8.1 8.1 5 12 5C13.6 5 15 5.5 16.2 6.4L14 8.6C13.4 8.2 12.7 8 12 8C9.8 8 8 9.8 8 12C8 13.1 8.5 14.1 9.2 14.8Z" />
    </svg>
  }
}
