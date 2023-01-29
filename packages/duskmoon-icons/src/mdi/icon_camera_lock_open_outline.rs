#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CameraLockOpenOutline)]
pub fn r#icon_camera_lock_open_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 20H4C2.9 20 2 19.1 2 18V6C2 4.9 2.9 4 4 4H7.2L9 2H15L16.8 4H20C21.1 4 22 4.9 22 6V11C21.4 10.6 20.7 10.2 20 10.1V6H16L14.2 4H9.9L8 6H4V18H13V20M12 7C9.2 7 7 9.2 7 12S9.2 17 12 17C12.5 17 12.9 16.9 13.4 16.8C13.6 16.4 13.9 16 14.2 15.8V14.6C14.2 14.4 14.2 14.2 14.3 14.1C13.7 14.7 12.9 15.1 12 15.1C10.3 15.1 9 13.8 9 12.1S10.3 9.1 12 9.1 15 10.4 15 12.1V12.2C15.4 11.6 16.1 11.1 16.8 10.7C16.1 8.5 14.3 7 12 7M21.8 17H17.5V14.5C17.5 13.7 18.2 13.2 19 13.2S20.5 13.7 20.5 14.5V15H21.8V14.5C21.8 13.1 20.4 12 19 12S16.2 13.1 16.2 14.5V17C15.6 17 15 17.6 15 18.2V21.7C15 22.4 15.6 23 16.2 23H21.7C22.4 23 23 22.4 23 21.8V18.3C23 17.6 22.4 17 21.8 17Z" />
    </svg>
  }
}
