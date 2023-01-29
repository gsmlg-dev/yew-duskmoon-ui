#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FilePercent)]
pub fn r#icon_file_percent(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14,2L20,8V20A2,2 0 0,1 18,22H6A2,2 0 0,1 4,20V4A2,2 0 0,1 6,2H14M7.37,20L15,12.35L13.65,11L6,18.65L7.37,20M13,9H18.5L13,3.5V9M7.5,11A1.5,1.5 0 0,0 6,12.5A1.5,1.5 0 0,0 7.5,14A1.5,1.5 0 0,0 9,12.5A1.5,1.5 0 0,0 7.5,11M13.5,17A1.5,1.5 0 0,0 12,18.5A1.5,1.5 0 0,0 13.5,20A1.5,1.5 0 0,0 15,18.5A1.5,1.5 0 0,0 13.5,17Z" />
    </svg>
  }
}
