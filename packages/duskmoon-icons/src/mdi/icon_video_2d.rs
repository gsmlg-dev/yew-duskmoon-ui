#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Video2d)]
pub fn r#icon_video_2d(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 7H16C17.66 7 19 8.34 19 10V14C19 15.66 17.66 17 16 17H13V7M16 15C16.55 15 17 14.55 17 14V10C17 9.45 16.55 9 16 9H15V15H16M5 7H9C10.11 7 11 7.9 11 9V11C11 12.11 10.11 13 9 13H7V15H11V17H5V13C5 11.9 5.9 11 7 11H9V9H5V7Z" />
    </svg>
  }
}
