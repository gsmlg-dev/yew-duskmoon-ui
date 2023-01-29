#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Webrtc)]
pub fn r#icon_webrtc(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 2C14.44 2 16.5 3.75 16.91 6.07L17.75 6C20.5 6 22.75 8.24 22.75 11C22.75 12.89 21.7 14.53 20.16 15.38C20.54 16.09 20.75 16.89 20.75 17.75C20.75 20.5 18.5 22.75 15.75 22.75C14.26 22.75 12.92 22.1 12 21.06C11.08 22.1 9.74 22.75 8.25 22.75C5.5 22.75 3.25 20.5 3.25 17.75C3.25 16.89 3.47 16.09 3.84 15.38C2.3 14.53 1.25 12.89 1.25 11C1.25 8.24 3.5 6 6.25 6L7.09 6.07C7.5 3.75 9.56 2 12 2M6.75 20.25L13.66 17H17C17.55 17 18 16.55 18 16V9C18 8.45 17.55 8 17 8H7C6.45 8 6 8.45 6 9V16C6 16.55 6.45 17 7 17H7.77L6.75 20.25Z" />
    </svg>
  }
}
