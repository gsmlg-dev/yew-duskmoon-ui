#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FilterMultiple)]
pub fn r#icon_filter_multiple(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3.46 5C3.25 5 3.04 5.08 2.87 5.21C2.43 5.55 2.35 6.18 2.69 6.61L2.69 6.62L8 13.42V19.41L10.29 21.71C10.68 22.1 11.32 22.1 11.71 21.71C12.1 21.32 12.1 20.68 11.71 20.29L10 18.59V12.73L4.27 5.39C4.08 5.14 3.78 5 3.46 5M16 12V19.88C16.04 20.18 15.94 20.5 15.71 20.71C15.32 21.1 14.69 21.1 14.3 20.71L12.29 18.7C12.06 18.47 11.96 18.16 12 17.87V12H11.97L6.21 4.62C5.87 4.19 5.95 3.56 6.38 3.22C6.57 3.08 6.78 3 7 3H21C21.22 3 21.43 3.08 21.62 3.22C22.05 3.56 22.13 4.19 21.79 4.62L16.03 12H16Z" />
    </svg>
  }
}
