#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SackPercent)]
pub fn r#icon_sack_percent(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8,7L6,2L10,4L12,2L14,4L18,2L16,7H8M16,22C10,22 8,22 8,22C2,22 3,18 3,18C3,18 4,11 8,9H16C20,11 21,18 21,18C21,18 22,22 16,22M7.5,12.5A1.5,1.5 0 0,0 9,14A1.5,1.5 0 0,0 10.5,12.5A1.5,1.5 0 0,0 9,11A1.5,1.5 0 0,0 7.5,12.5M16.5,18.5A1.5,1.5 0 0,0 15,17A1.5,1.5 0 0,0 13.5,18.5A1.5,1.5 0 0,0 15,20A1.5,1.5 0 0,0 16.5,18.5M16.5,12.35L15.15,11L7.5,18.65L8.87,20L16.5,12.35Z" />
    </svg>
  }
}
