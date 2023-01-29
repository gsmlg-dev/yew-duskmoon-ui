#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SimOffOutline)]
pub fn r#icon_sim_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.11 21.46L2.39 1.73L1.11 3L5.06 6.95L4 8V20C4 21.11 4.89 22 6 22H18C18.58 22 19.1 21.75 19.46 21.35L20.84 22.73L22.11 21.46M18 20H6V8.83L6.47 8.36L18 19.89V20M10.83 4H18V14.8L20 16.8V4C20 2.9 19.11 2 18 2H10L7.6 4.4L9 5.81L10.83 4Z" />
    </svg>
  }
}
