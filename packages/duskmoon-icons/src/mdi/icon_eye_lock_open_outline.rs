#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_EyeLockOpenOutline)]
pub fn r#icon_eye_lock_open_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 19.5C7 19.5 2.7 16.4 1 12C2.7 7.6 7 4.5 12 4.5S21.3 7.6 23 12C22.9 12.4 22.7 12.7 22.5 13.1C22 11.9 21 10.9 19.7 10.4C17.9 8 15.1 6.5 12 6.5C8.2 6.5 4.8 8.6 3.2 12C4.9 15.4 8.3 17.5 12 17.5H12.1C12 17.7 12 18 12 18.2V19.5M12 9C10.3 9 9 10.3 9 12S10.3 15 12 15C12.4 15 12.8 14.9 13.2 14.7V14.5C13.2 13.2 13.9 12 14.9 11.1C14.5 9.9 13.4 9 12 9M20.8 17H16.5V14.5C16.5 13.7 17.2 13.2 18 13.2S19.5 13.7 19.5 14.5V15H20.8V14.5C20.8 13.1 19.4 12 18 12S15.2 13.1 15.2 14.5V17C14.6 17 14 17.6 14 18.2V21.7C14 22.4 14.6 23 15.2 23H20.7C21.4 23 22 22.4 22 21.8V18.3C22 17.6 21.4 17 20.8 17Z" />
    </svg>
  }
}
