#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CookieRefresh)]
pub fn r#icon_cookie_refresh(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 16C11.53 16 12 16.28 12.26 16.69C13.04 14 15.54 12 18.5 12C19.38 12 20.21 12.18 21 12.5C21 12.33 21 12.17 21 12C21 11.5 20.96 11 20.87 10.5C20.6 10 20 10 20 10H18V9C18 8 17 8 17 8H15V7C15 6 14 6 14 6H13V4C13 3 12 3 12 3C7.03 3 3 7.03 3 12S7.03 21 12 21C12.17 21 12.33 21 12.5 21C12.19 20.24 12 19.44 12 18.61C11.74 18.85 11.39 19 11 19C10.17 19 9.5 18.33 9.5 17.5S10.17 16 11 16M13 12.5C13 13.33 12.33 14 11.5 14S10 13.33 10 12.5 10.67 11 11.5 11 13 11.67 13 12.5M6.5 13C5.67 13 5 12.33 5 11.5S5.67 10 6.5 10 8 10.67 8 11.5 7.33 13 6.5 13M8 7.5C8 6.67 8.67 6 9.5 6S11 6.67 11 7.5 10.33 9 9.5 9 8 8.33 8 7.5M18 18.5L19.77 16.73C19.32 16.28 18.69 16 18 16C16.62 16 15.5 17.12 15.5 18.5S16.62 21 18 21C18.82 21 19.54 20.61 20 20H21.71C21.12 21.47 19.68 22.5 18 22.5C15.79 22.5 14 20.71 14 18.5S15.79 14.5 18 14.5C19.11 14.5 20.11 14.95 20.83 15.67L22 14.5V18.5H18Z" />
    </svg>
  }
}
