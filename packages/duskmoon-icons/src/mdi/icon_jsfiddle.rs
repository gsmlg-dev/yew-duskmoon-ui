#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Jsfiddle)]
pub fn r#icon_jsfiddle(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17.34 13.7C17.34 15 16.23 16.04 14.86 16.04C13.65 16.04 12.64 15 11.75 14.04L11.5 13.79C11.5 13.76 11.47 13.73 11.45 13.7C10.74 12.96 9.96 12.22 9.21 12.22C8.32 12.22 7.6 12.88 7.6 13.69C7.6 14.5 8.32 15.17 9.21 15.17C9.97 15.17 10.35 14.75 10.63 14.45L10.7 14.37C10.86 14.2 11.14 14.19 11.31 14.35C11.5 14.5 11.5 14.79 11.33 14.96L11.27 15.03C10.94 15.38 10.35 16.03 9.21 16.03C7.84 16.03 6.73 15 6.73 13.69C6.73 12.4 7.84 11.35 9.21 11.35C10.42 11.35 11.43 12.41 12.32 13.35L12.56 13.6C12.58 13.63 12.6 13.66 12.62 13.69C13.33 14.43 14.11 15.17 14.86 15.17C15.75 15.17 16.47 14.5 16.47 13.69C16.47 12.88 15.75 12.22 14.86 12.22C14.11 12.22 13.72 12.64 13.44 12.94L13.37 13C13.21 13.19 12.94 13.2 12.76 13.04C12.59 12.87 12.58 12.6 12.74 12.42L12.81 12.36C13.13 12 13.73 11.36 14.86 11.36C16.23 11.36 17.34 12.4 17.34 13.7M22 14.85C22 15.96 21.57 17 20.78 17.79C20 18.57 18.95 19 17.84 19H6.28C3.96 18.96 2.07 17.06 2.07 14.75C2.07 13.37 2.76 12.07 3.89 11.28C3.85 11.09 3.83 10.9 3.83 10.7C3.83 9.03 5.2 7.67 6.88 7.67C7.39 7.67 7.88 7.79 8.32 8.03C9.41 6.17 11.43 5 13.6 5C16.97 5 19.7 7.72 19.7 11.07L19.7 11.14C21.11 11.84 22 13.27 22 14.85M21.13 14.85C21.13 13.5 20.33 12.32 19.09 11.81C18.92 11.74 18.81 11.57 18.82 11.38L18.83 11.29C18.83 11.22 18.84 11.14 18.84 11.07C18.84 8.2 16.5 5.87 13.6 5.87C11.6 5.87 9.74 7.03 8.87 8.83C8.82 8.95 8.71 9.04 8.58 9.07C8.46 9.1 8.32 9.07 8.22 9C7.83 8.69 7.37 8.53 6.88 8.53C5.68 8.53 4.7 9.5 4.7 10.7C4.7 10.92 4.73 11.14 4.8 11.34C4.86 11.54 4.78 11.75 4.61 11.85C3.56 12.47 2.94 13.55 2.94 14.75C2.94 16.59 4.44 18.1 6.29 18.13H17.83C18.72 18.13 19.54 17.79 20.16 17.17C20.79 16.55 21.13 15.73 21.13 14.85Z" />
    </svg>
  }
}
