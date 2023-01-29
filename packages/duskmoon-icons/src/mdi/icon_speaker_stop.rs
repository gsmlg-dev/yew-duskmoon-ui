#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SpeakerStop)]
pub fn r#icon_speaker_stop(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.28 19.81C11.87 19.92 11.45 20 11 20C8.24 20 6 17.76 6 15S8.24 10 11 10C12.89 10 14.5 11.06 15.37 12.61C16.16 12.23 17.06 12 18 12V4C18 2.89 17.1 2 16 2H6C4.89 2 4 2.89 4 4V20C4 21.11 4.89 22 6 22H13.54C13 21.37 12.54 20.63 12.28 19.81M11 4C12.11 4 13 4.89 13 6S12.11 8 11 8C9.89 8 9 7.1 9 6C9 4.89 9.89 4 11 4M13.74 13.78C12.7 14.82 12.06 16.24 12 17.81C11.69 17.93 11.36 18 11 18C9.34 18 8 16.66 8 15S9.34 12 11 12C12.22 12 13.27 12.73 13.74 13.78M21 15H15V21H21V15Z" />
    </svg>
  }
}
