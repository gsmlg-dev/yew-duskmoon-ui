#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CameraLockOpen)]
pub fn r#icon_camera_lock_open(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 12C14.6 12.6 14.4 13.3 14.3 14C13.7 14.6 12.9 15 12 15C10.3 15 9 13.7 9 12S10.3 9 12 9 15 10.3 15 12M13 18.2C13 17.7 13.1 17.2 13.4 16.8C12.9 16.9 12.5 17 12 17C9.2 17 7 14.8 7 12S9.2 7 12 7C14.3 7 16.1 8.5 16.8 10.6C17.5 10.3 18.2 10 19 10C20.1 10 21.2 10.4 22 11V6C22 4.9 21.1 4 20 4H17L15 2H9L7 4H4C2.9 4 2 4.9 2 6V18C2 19.1 2.9 20 4 20H13V18.2M21.8 17H17.5V14.5C17.5 13.7 18.2 13.2 19 13.2S20.5 13.7 20.5 14.5V15H21.8V14.5C21.8 13.1 20.4 12 19 12S16.2 13.1 16.2 14.5V17C15.6 17 15 17.6 15 18.2V21.7C15 22.4 15.6 23 16.2 23H21.7C22.4 23 23 22.4 23 21.8V18.3C23 17.6 22.4 17 21.8 17Z" />
    </svg>
  }
}
