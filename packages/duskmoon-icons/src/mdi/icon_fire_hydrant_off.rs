#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FireHydrantOff)]
pub fn r#icon_fire_hydrant_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.84 22.73L22.11 21.46L2.39 1.73L1.11 3L8 9.89V20C6.9 20 6 20.9 6 22H18C18 20.9 17.11 20 16 20V17.89L20.84 22.73M10.22 12.11L12.89 14.78C12.62 14.92 12.32 15 12 15C10.9 15 10 14.11 10 13C10 12.68 10.08 12.38 10.22 12.11M11.2 8L8.44 5.24C8.94 4.21 9.87 3.43 11 3.14V2H13V3.14C14.4 3.5 15.5 4.6 15.86 6H18V8H11.2M19 11V12H20V14H19V15H18.2L17 13.8V11H19M5 15V14H4V12H5V11H7V15H5M16 12.8L12.2 9H16V12.8Z" />
    </svg>
  }
}
