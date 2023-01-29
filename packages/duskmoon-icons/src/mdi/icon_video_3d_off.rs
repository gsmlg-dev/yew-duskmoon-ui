#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Video3dOff)]
pub fn r#icon_video_3d_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2.61 2L21.35 20.74L19.94 22.15L14.8 17H13V15.2L11 13.23V15.21C10.93 15.68 10.74 16.08 10.41 16.41C10.08 16.73 9.68 16.93 9.21 17H5V15H9V13H6V11H8.8L6.79 9H5V7.23L1.2 3.41L2.61 2M16 7H16.22C17 7.07 17.63 7.36 18.14 7.88C18.65 8.39 18.93 9 19 9.76V14.24C18.95 14.63 18.87 15 18.72 15.3L17 13.6V9.85C16.95 9.63 16.84 9.44 16.69 9.28C16.53 9.13 16.34 9.03 16.12 9H15V11.59L13 9.57V7H16Z" />
    </svg>
  }
}
