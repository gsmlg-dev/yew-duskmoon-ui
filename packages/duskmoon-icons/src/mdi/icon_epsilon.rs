#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Epsilon)]
pub fn r#icon_epsilon(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15.05 7.78L16.15 6.18C16.15 6.18 14.91 5 12.77 5C10.04 5 8.35 6.84 8.35 8.76C8.35 10.68 10.08 11.69 10.08 11.69C10.08 11.69 8 12.38 8 15C8 17.63 10.14 19 12.44 19C15.38 19 17 17.04 17 17.04L15.6 15.5C15.6 15.5 14.14 16.87 12.59 16.87C10.66 16.87 10.21 15.69 10.21 14.92C10.21 13.87 10.54 12.65 13.83 12.65L13.82 10.77C13.82 10.77 10.44 11.11 10.44 8.78C10.44 7.21 11.9 6.92 12.64 6.92C14.28 6.92 15.05 7.78 15.05 7.78" />
    </svg>
  }
}
