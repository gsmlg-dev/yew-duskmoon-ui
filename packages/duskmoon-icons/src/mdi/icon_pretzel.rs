#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Pretzel)]
pub fn r#icon_pretzel(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5.15 15.84C3.81 14.27 3 12.23 3 10V9.97C3 7.22 5.25 5 8 5C9.64 5 11.09 5.79 12 7C12.91 5.79 14.37 5 16 5C18.76 5 21 7.24 21 10C21 12.23 20.19 14.27 18.85 15.84L20.21 17.2L18.79 18.61L17.39 17.21C15.89 18.33 14 19 12 19C10 19 8.11 18.33 6.61 17.21L5.21 18.61L3.79 17.2L5.15 15.84M15.96 15.77L12 11.82L8.04 15.77C9.17 16.55 10.53 17 12 17C13.47 17 14.83 16.55 15.96 15.77M11 10C11 8.34 9.65 7 8 7C6.34 7 5 8.34 5 10C5 11.68 5.59 13.21 6.57 14.42L11 10M17.43 14.42C18.41 13.21 19 11.68 19 10V10C19 8.33 17.65 7 16 7C14.35 7 13 8.34 13 10L17.43 14.42Z" />
    </svg>
  }
}
