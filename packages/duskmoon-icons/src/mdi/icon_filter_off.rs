#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FilterOff)]
pub fn r#icon_filter_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2.39 1.73L1.11 3L9.5 11.37L9.97 12H10V17.87C9.96 18.16 10.06 18.47 10.29 18.7L12.3 20.71C12.69 21.1 13.32 21.1 13.71 20.71C13.94 20.5 14.04 20.18 14 19.88V15.89L20.84 22.73L22.11 21.46L14 13.35L9.41 8.76L4.15 3.5L2.39 1.73M6.21 3L14.54 11.34L19.79 4.62C20.13 4.19 20.05 3.56 19.62 3.22C19.43 3.08 19.22 3 19 3H6.21Z" />
    </svg>
  }
}
