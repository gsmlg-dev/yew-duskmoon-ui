#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_HeartCog)]
pub fn r#icon_heart_cog(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 19C12 19.71 12.12 20.4 12.32 21.06L12 21.35L10.55 20.03C5.4 15.36 2 12.27 2 8.5C2 5.41 4.42 3 7.5 3C9.24 3 10.91 3.81 12 5.08C13.09 3.81 14.76 3 16.5 3C19.58 3 22 5.41 22 8.5C22 9.8 21.59 11 20.84 12.25C20.25 12.09 19.63 12 19 12C15.14 12 12 15.14 12 19M23.83 20.64L22.83 22.37C22.76 22.5 22.63 22.5 22.5 22.5L21.27 22C21 22.18 20.73 22.34 20.43 22.47L20.24 23.79C20.22 23.91 20.11 24 20 24H18C17.86 24 17.76 23.91 17.74 23.79L17.55 22.47C17.24 22.35 16.96 22.18 16.7 22L15.46 22.5C15.34 22.5 15.21 22.5 15.15 22.37L14.15 20.64C14.09 20.53 14.12 20.4 14.21 20.32L15.27 19.5C15.25 19.33 15.24 19.17 15.24 19S15.25 18.67 15.27 18.5L14.21 17.68C14.11 17.6 14.09 17.47 14.15 17.36L15.15 15.63C15.22 15.5 15.35 15.5 15.46 15.5L16.7 16C16.96 15.82 17.25 15.66 17.55 15.53L17.74 14.21C17.76 14.09 17.87 14 18 14H20C20.11 14 20.22 14.09 20.23 14.21L20.42 15.53C20.73 15.65 21 15.82 21.27 16L22.5 15.5C22.63 15.5 22.76 15.5 22.82 15.63L23.82 17.36C23.88 17.47 23.85 17.6 23.76 17.68L22.7 18.5C22.73 18.67 22.74 18.83 22.74 19S22.72 19.33 22.7 19.5L23.77 20.32C23.86 20.4 23.89 20.53 23.83 20.64M20.5 19C20.5 18.17 19.83 17.5 19 17.5S17.5 18.17 17.5 19 18.16 20.5 19 20.5C19.83 20.5 20.5 19.83 20.5 19Z" />
    </svg>
  }
}
