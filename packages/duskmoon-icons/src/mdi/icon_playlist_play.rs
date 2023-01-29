#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PlaylistPlay)]
pub fn r#icon_playlist_play(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 10H14V12H3V10M3 6H14V8H3V6M3 14H10V16H3V14M16 13V21L22 17L16 13Z" />
    </svg>
  }
}
