#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CameraMarker)]
pub fn r#icon_camera_marker(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.5 16.8C17.8 16.8 17.3 16.2 17.3 15.6C17.3 14.9 17.9 14.4 18.5 14.4S19.7 15 19.7 15.6C19.8 16.2 19.2 16.8 18.5 16.8M18.5 12C16.6 12 15 13.6 15 15.5C15 18.1 18.5 22 18.5 22S22 18.1 22 15.5C22 13.6 20.4 12 18.5 12M9 2L7 4H4C2.9 4 2 4.9 2 6V18C2 19.1 2.9 20 4 20H14.5C13.9 19.1 13.4 18.1 13.1 16.9C12.8 17 12.4 17 12 17C9.2 17 7 14.8 7 12S9.2 7 12 7C14.1 7 16 8.3 16.7 10.3C17.3 10.1 17.9 10 18.5 10C19.8 10 21 10.5 22 11.3V6C22 4.9 21.1 4 20 4H17L15 2H9M11.9 9C10.3 9 9 10.4 9 12C9 13.7 10.3 15 12 15C12.4 15 12.7 14.9 13 14.8C13.2 13.4 13.9 12.2 14.9 11.3C14.6 10 13.4 9 11.9 9C12 9 12 9 11.9 9Z" />
    </svg>
  }
}
