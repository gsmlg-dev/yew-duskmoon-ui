#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_StadiumOutline)]
pub fn r#icon_stadium_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7 5L3 7V3L7 5M18 3V7L22 5L18 3M11 2V6L15 4L11 2M13 18H11V22C5.9 21.9 2 20.6 2 19V10C2 8.3 6.5 7 12 7S22 8.3 22 10V19C22 20.6 18 21.9 13 22V18M5 10C6.4 10.5 8.8 11 12 11S17.6 10.5 19 10C19 9.8 16.2 9 12 9S5 9.9 5 10M20 11.8C18.2 12.5 15.3 13 12 13S5.8 12.5 4 11.8V18.6C4.6 19 6.4 19.6 9 19.9V16H15V19.9C17.6 19.6 19.4 19 20 18.6V11.8Z" />
    </svg>
  }
}
