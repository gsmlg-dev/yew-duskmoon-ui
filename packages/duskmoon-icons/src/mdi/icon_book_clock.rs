#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BookClock)]
pub fn r#icon_book_clock(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.5 17.25L19.36 18.94L18.61 20.16L15 18V13H16.5V17.25M23 17C23 20.87 19.87 24 16 24C14.09 24 12.37 23.24 11.11 22H6C4.89 22 4 21.11 4 20V4C4 2.9 4.89 2 6 2H7V9L9.5 7.5L12 9V2H18C19.1 2 20 2.89 20 4V11.26C21.81 12.53 23 14.62 23 17M21 17C21 14.24 18.76 12 16 12S11 14.24 11 17 13.24 22 16 22 21 19.76 21 17Z" />
    </svg>
  }
}
