#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DeskLampOn)]
pub fn r#icon_desk_lamp_on(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10.85 2L9.18 4.5L10.32 5.25L7.14 10H7C5.9 10 5 10.9 5 12C5 12.94 5.66 13.75 6.58 13.95L10.62 20H7V22H17V20H13L8.53 13.28C8.83 12.92 9 12.47 9 12C9 11.7 8.93 11.4 8.8 11.13L12 6.37C11.78 8.05 12.75 9.89 14.45 11L18.89 4.37C17.2 3.24 15.12 3.04 13.65 3.87L10.85 2M18.33 7L16.67 9.5C17.35 9.95 18.29 9.77 18.75 9.08C19.21 8.39 19 7.46 18.33 7M21.7 12.58L19.58 10.45L20.28 9.75L22.4 11.87L21.7 12.58M23 7H20V6H23V7M16 14V11H17V14H16Z" />
    </svg>
  }
}
