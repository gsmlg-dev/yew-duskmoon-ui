#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_RobotVacuumVariantOff)]
pub fn r#icon_robot_vacuum_variant_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.8 22.7L17.3 19.2C15.8 20.3 14 21 12 21C7 21 3 17 3 12V9H5V12C5 15.9 8.1 19 12 19C13.4 19 14.8 18.5 15.9 17.8L14 15.9C13.5 16.5 12.8 17 12 17C10.6 17 9.5 15.9 9.5 14.5C9.5 13.7 9.9 12.9 10.6 12.5L5 6.9V7H3V4.9L1.1 3L2.4 1.7L22.1 21.4L20.8 22.7M19 5V7H21V5C21 3.9 20.1 3 19 3H6.2L8.2 5H19M19 12C19 13.1 18.7 14.1 18.3 15.1L19.8 16.6C20.6 15.3 21 13.7 21 12V9H19V12M16 9V7H10.2L12.2 9H16Z" />
    </svg>
  }
}
