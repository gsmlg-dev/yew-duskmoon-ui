#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FireHydrant)]
pub fn r#icon_fire_hydrant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 15V14H4V12H5V11H7V15H5M19 12V11H17V15H19V14H20V12H19M18 6H15.86C15.5 4.6 14.4 3.5 13 3.14V2H11V3.14C9.6 3.5 8.5 4.6 8.14 6H6V8H18V6M18 22H6C6 20.9 6.9 20 8 20V9H16V20C17.11 20 18 20.9 18 22M10 13C10 14.11 10.9 15 12 15S14 14.11 14 13 13.11 11 12 11 10 11.9 10 13Z" />
    </svg>
  }
}