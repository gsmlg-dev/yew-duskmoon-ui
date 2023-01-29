#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_EmailSearchOutline)]
pub fn r#icon_email_search_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.5 11C19 11 21 13 21 15.5C21 16.38 20.75 17.21 20.31 17.9L23.39 21L22 22.39L18.88 19.32C18.19 19.75 17.37 20 16.5 20C14 20 12 18 12 15.5S14 11 16.5 11M16.5 13C15.12 13 14 14.12 14 15.5S15.12 18 16.5 18 19 16.88 19 15.5 17.88 13 16.5 13M10.5 18H3V8L10.62 12.76C11.65 10.54 13.9 9 16.5 9C16.77 9 17.04 9 17.31 9.06L19 8V9.5C19.75 9.81 20.42 10.27 21 10.82V6C21 4.9 20.1 4 19 4H3C1.9 4 1 4.9 1 6V18C1 19.1 1.9 20 3 20H11.82C11.27 19.42 10.82 18.75 10.5 18M19 6L11 11L3 6H19Z" />
    </svg>
  }
}
