#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Kangaroo)]
pub fn r#icon_kangaroo(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.8 6.59L19 7.97V10.04L17.69 11.61L17.31 13.25L16 14L15.5 13.12L16.44 12.59L16.7 11.45L16.7 11.45L16.77 11.15L16.77 11.15V11.15L17.47 10.31C17.65 10.1 17.62 9.78 17.41 9.61C17.2 9.43 16.88 9.46 16.7 9.67L15.85 10.69L15.56 11.93C15.38 11.96 15.2 12 15 12C14.31 12 13.68 11.76 13.23 11.4C13.15 12.7 12.73 13.81 12.13 14.43L10.5 16.19L9.96 19.79L8.07 21L7.53 20.17L9.04 19.19L9.5 15.92L9.5 15.91L10 14.54C9.47 14.08 9.08 13.28 8.88 12.3L8.71 12.61C8.35 13.25 8 13.9 7.56 14.56C7.11 15.2 6.66 15.9 5.78 16.44C5.34 16.72 4.7 16.84 4.17 16.73C3.61 16.61 3.14 16.26 2.86 15.89C2.31 15.13 2.15 14.35 2 13.62L2.97 13.36C3.2 14 3.5 14.65 3.88 15C4.26 15.33 4.57 15.26 4.84 15C5.16 14.73 5.47 14.15 5.73 13.55C6 12.94 6.22 12.28 6.45 11.6C6.93 10.24 7.39 8.82 8.2 7.36C8.62 6.64 9.13 5.89 9.92 5.22C10.7 4.55 11.73 4 13 4S15.7 5.22 16.58 6.34C17 6.89 17.87 6.82 18.22 6.21L19.14 4.63C19 4.58 18.89 4.5 18.79 4.4C18.4 4 18.4 3.38 18.79 3L19.93 4.13C19.96 4.13 20 4.11 20 4.11C20.41 4.11 20.75 4.35 20.91 4.69L22 7L21 7.5L19.8 6.59Z" />
    </svg>
  }
}
