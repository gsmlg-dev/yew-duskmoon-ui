#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CeilingFan)]
pub fn r#icon_ceiling_fan(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8 3V5H11V10.27C10.38 10.63 10 11.29 10 12V13H14V12C14 11.29 13.62 10.63 13 10.27V5H16V3H8M6 12C3.79 12 2 12.67 2 13.5S3.79 15 6 15 10 14.33 10 13.5 8.21 12 6 12M18 12C15.79 12 14 12.67 14 13.5S15.79 15 18 15 22 14.33 22 13.5 20.21 12 18 12M10 14V15C10 15.72 10.38 16.38 11 16.73C11.62 17.09 12.38 17.09 13 16.73C13.62 16.38 14 15.71 14 15V14H10Z" />
    </svg>
  }
}
