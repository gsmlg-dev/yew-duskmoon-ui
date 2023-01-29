#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FractionOneHalf)]
pub fn r#icon_fraction_one_half(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5.79 21.61L4.21 20.39L18.21 2.39L19.79 3.61L5.79 21.61M4 2V4H6V12H8V2H4M15 12V14H19V16H17C15.9 16 15 16.9 15 18V22H21V20H17V18H19C20.11 18 21 17.11 21 16V14C21 12.9 20.11 12 19 12H15Z" />
    </svg>
  }
}
