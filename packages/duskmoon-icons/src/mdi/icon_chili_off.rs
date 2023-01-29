#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ChiliOff)]
pub fn r#icon_chili_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 8.28C15.6 8.63 16 9.27 16 10V12.8L11.5 8.29L12 8L13.75 9L15 8.28M12 6.5L13.75 7.5L15.27 6.63C14.72 5.66 13.91 4.94 12.97 4.65C12.79 3.16 11.54 2 10 2V4C10.44 4 10.8 4.29 10.94 4.69C10.26 4.92 9.66 5.37 9.17 5.97L10.54 7.34L12 6.5M2.39 1.73L1.11 3L8 9.9C8 9.94 8 9.97 8 10V11C8 20 16 22 16 22V17.89L20.84 22.73L22.11 21.46L2.39 1.73Z" />
    </svg>
  }
}
