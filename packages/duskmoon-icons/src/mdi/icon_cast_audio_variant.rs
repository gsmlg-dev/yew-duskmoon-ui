#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CastAudioVariant)]
pub fn r#icon_cast_audio_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.5 12C14.5 10.62 13.38 9.5 12 9.5C10.62 9.5 9.5 10.62 9.5 12C9.5 13 10.09 13.84 10.93 14.25L9.82 15.35C8.73 14.64 8 13.41 8 12C8 9.79 9.79 8 12 8C14.21 8 16 9.79 16 12C16 13.41 15.27 14.64 14.18 15.35L13.08 14.25C13.92 13.84 14.5 13 14.5 12M12 2C6.5 2 2 6.5 2 12C2 15.06 3.38 17.8 5.54 19.63L6.61 18.57C4.71 17 3.5 14.64 3.5 12C3.5 7.31 7.31 3.5 12 3.5C16.69 3.5 20.5 7.31 20.5 12C20.5 14.64 19.29 17 17.39 18.57L18.46 19.63C20.62 17.79 22 15.06 22 12C22 6.5 17.5 2 12 2M19 12C19 8.13 15.87 5 12 5C8.13 5 5 8.13 5 12C5 14.23 6.05 16.22 7.68 17.5L8.75 16.43C7.39 15.42 6.5 13.82 6.5 12C6.5 8.97 8.97 6.5 12 6.5C15.03 6.5 17.5 8.97 17.5 12C17.5 13.82 16.61 15.42 15.25 16.43L16.33 17.5C17.95 16.22 19 14.23 19 12M6 22H18L12 16L6 22Z" />
    </svg>
  }
}
