#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PhoneClock)]
pub fn r#icon_phone_clock(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15.9 1C12.1 1.1 9 4.2 9 8C9 11.9 12.1 15 16 15S23 11.9 23 8 19.9 1 15.9 1C16 1 15.9 1 15.9 1M16 3C18.8 3 21 5.2 21 8S18.8 13 16 13 11 10.8 11 8 13.2 3 16 3M15 4V9L18.6 11.2L19.4 10L16.5 8.3V4H15M4.6 12.8C6 15.6 8.4 18 11.2 19.4L13.4 17.2C13.7 16.9 14.1 16.8 14.4 17C15.5 17.4 16.7 17.6 18 17.6C18.5 17.6 19 18.1 19 18.6V22C19 22.5 18.5 23 18 23C8.6 23 1 15.4 1 6C1 5.4 1.5 5 2 5H5.5C6.1 5 6.5 5.4 6.5 6C6.5 7.2 6.7 8.4 7.1 9.6C7.2 10 7.1 10.3 6.9 10.6L4.6 12.8" />
    </svg>
  }
}
