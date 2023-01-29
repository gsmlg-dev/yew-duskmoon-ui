#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ZodiacLibra)]
pub fn r#icon_zodiac_libra(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20,16V18H13V15.91C16,15.36 17.96,12.5 17.41,9.5C16.86,6.5 14,4.54 11,5.09C8,5.65 6.04,8.5 6.59,11.5C7,13.74 8.76,15.5 11,15.91V18H4V16H6.92C5.37,14.59 4.5,12.59 4.5,10.5A7.5,7.5 0 0,1 12,3A7.5,7.5 0 0,1 19.5,10.5C19.5,12.59 18.63,14.59 17.08,16H20M20,19H4V21H20V19Z" />
    </svg>
  }
}
