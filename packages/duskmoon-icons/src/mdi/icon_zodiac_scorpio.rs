#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ZodiacScorpio)]
pub fn r#icon_zodiac_scorpio(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17.71,15.29L16.29,16.71L17.59,18H16A2,2 0 0,1 14,16V6A3,3 0 0,0 11,3C10.25,3 9.55,3.29 9,3.78C7.86,2.76 6.14,2.76 5,3.78C4.45,3.28 3.74,3 3,3V5A1,1 0 0,1 4,6V16H6V6A1,1 0 0,1 7,5A1,1 0 0,1 8,6V16H10V6A1,1 0 0,1 11,5A1,1 0 0,1 12,6V16A4,4 0 0,0 16,20H17.59L16.29,21.29L17.71,22.71L21.41,19L17.71,15.29Z" />
    </svg>
  }
}
