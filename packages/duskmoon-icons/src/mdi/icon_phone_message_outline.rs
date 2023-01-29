#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PhoneMessageOutline)]
pub fn r#icon_phone_message_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 15.5C18.75 15.5 17.55 15.3 16.43 14.93C16.33 14.9 16.22 14.88 16.12 14.88C15.86 14.88 15.61 15 15.41 15.17L13.21 17.37C10.38 15.93 8.06 13.62 6.62 10.79L8.82 8.58C9.1 8.31 9.18 7.92 9.07 7.57C8.7 6.45 8.5 5.25 8.5 4C8.5 3.45 8.05 3 7.5 3H4C3.45 3 3 3.45 3 4C3 13.39 10.61 21 20 21C20.55 21 21 20.55 21 20V16.5C21 15.95 20.55 15.5 20 15.5M5.03 5H6.53C6.6 5.88 6.75 6.75 7 7.59L5.79 8.8C5.38 7.59 5.12 6.32 5.03 5M19 18.97C17.68 18.88 16.4 18.62 15.2 18.21L16.4 17C17.25 17.25 18.12 17.4 19 17.46V18.97M12 3V13L15 10H21V3H12M19 8H14V5H19V8Z" />
    </svg>
  }
}
