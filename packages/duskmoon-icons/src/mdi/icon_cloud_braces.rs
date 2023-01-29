#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CloudBraces)]
pub fn r#icon_cloud_braces(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.86 12.5C21.1 11.63 20.15 11.13 19 11C19 9.05 18.32 7.4 16.96 6.04C15.6 4.68 13.95 4 12 4C10.42 4 9 4.47 7.75 5.43S5.67 7.62 5.25 9.15C4 9.43 2.96 10.08 2.17 11.1S1 13.28 1 14.58C1 16.09 1.54 17.38 2.61 18.43C3.69 19.5 5 20 6.5 20H18.5C19.75 20 20.81 19.56 21.69 18.69C22.56 17.81 23 16.75 23 15.5C23 14.35 22.62 13.35 21.86 12.5M10.5 10H9V11C9 12.11 8.11 13 7 13C8.11 13 9 13.9 9 15V16H10.5V18H9C7.9 18 7 17.11 7 16V15C7 14.45 6.55 14 6 14H5.5V12H6C6.55 12 7 11.55 7 11V10C7 8.9 7.9 8 9 8H10.5V10M18.5 14H18C17.45 14 17 14.45 17 15V16C17 17.11 16.11 18 15 18H13.5V16H15V15C15 13.9 15.9 13 17 13C15.9 13 15 12.11 15 11V10H13.5V8H15C16.11 8 17 8.9 17 10V11C17 11.55 17.45 12 18 12H18.5V14Z" />
    </svg>
  }
}
