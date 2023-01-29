#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ImageLockOutline)]
pub fn r#icon_image_lock_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.8 16V14.5C21.8 13.1 20.4 12 19 12S16.2 13.1 16.2 14.5V16C15.6 16 15 16.6 15 17.2V20.7C15 21.4 15.6 22 16.2 22H21.7C22.4 22 23 21.4 23 20.8V17.3C23 16.6 22.4 16 21.8 16M20.5 16H17.5V14.5C17.5 13.7 18.2 13.2 19 13.2S20.5 13.7 20.5 14.5V16M5 3C3.9 3 3 3.9 3 5V19C3 20.11 3.9 21 5 21H13.03C13 20.9 13 20.8 13 20.7V19H5V5H19V10C19.69 10 20.37 10.16 21 10.42V5C21 3.9 20.11 3 19 3H5M13.96 12.29L11.21 15.83L9.25 13.47L6.5 17H13C13.08 16.14 13.46 15.46 13.96 14.96C14.03 14.89 14.13 14.85 14.2 14.79V14.5C14.2 13.95 14.3 13.44 14.47 12.97L13.96 12.29Z" />
    </svg>
  }
}
