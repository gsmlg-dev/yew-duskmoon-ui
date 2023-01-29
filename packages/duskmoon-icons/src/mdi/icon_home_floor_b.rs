#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_HomeFloorB)]
pub fn r#icon_home_floor_b(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,3L2,12H5V20H19V12H22L12,3M9,8H13A2,2 0 0,1 15,10V11.5A1.5,1.5 0 0,1 13.5,13A1.5,1.5 0 0,1 15,14.5V16A2,2 0 0,1 13,18H9V8M11,10V12H13V10H11M11,14V16H13V14H11Z" />
    </svg>
  }
}
