#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RobotExcited)]
pub fn r#icon_robot_excited(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 14H21C21 10.13 17.87 7 14 7H13V5.73C13.6 5.39 14 4.74 14 4C14 2.9 13.11 2 12 2S10 2.9 10 4C10 4.74 10.4 5.39 11 5.73V7H10C6.13 7 3 10.13 3 14H2C1.45 14 1 14.45 1 15V18C1 18.55 1.45 19 2 19H3V20C3 21.11 3.9 22 5 22H19C20.11 22 21 21.11 21 20V19H22C22.55 19 23 18.55 23 18V15C23 14.45 22.55 14 22 14M8.68 17.04L7.5 15.86L6.32 17.04L5.14 15.86L7.5 13.5L9.86 15.86L8.68 17.04M17.68 17.04L16.5 15.86L15.32 17.04L14.14 15.86L16.5 13.5L18.86 15.86L17.68 17.04Z" />
    </svg>
  }
}
