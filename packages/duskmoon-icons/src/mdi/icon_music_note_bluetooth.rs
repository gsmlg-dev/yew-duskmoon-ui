#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MusicNoteBluetooth)]
pub fn r#icon_music_note_bluetooth(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 9.85L19.14 7H18.64V10.79L16.35 8.5L15.64 9.21L18.43 12L15.64 14.79L16.35 15.5L18.64 13.21V17H19.14L22 14.15L19.85 12L22 9.85M19.64 8.91L20.58 9.85L19.64 10.79V8.91M20.58 14.15L19.64 15.09V13.21L20.58 14.15M9.64 3V13.55C9.05 13.21 8.37 13 7.64 13C5.43 13 3.64 14.79 3.64 17S5.43 21 7.64 21 11.64 19.21 11.64 17V7H15.64V3H9.64Z" />
    </svg>
  }
}
