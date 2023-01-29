#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_UmbrellaBeach)]
pub fn r#icon_umbrella_beach(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13.13 14.56L14.56 13.13L21 19.57L19.57 21L13.13 14.56M17.42 8.83L20.28 5.97C16.33 2 9.93 2 6 5.95C9.91 4.65 14.29 5.7 17.42 8.83M5.95 6C2 9.93 2 16.33 5.97 20.28L8.83 17.42C5.7 14.29 4.65 9.91 5.95 6M5.97 5.96L5.96 5.97C5.58 9 7.13 12.85 10.26 16L16 10.26C12.86 7.13 9 5.58 5.97 5.96Z" />
    </svg>
  }
}
