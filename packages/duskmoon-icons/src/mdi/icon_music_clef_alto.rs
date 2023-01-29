#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MusicClefAlto)]
pub fn r#icon_music_clef_alto(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 4H7V20H5M15.46 13H14.83L13.83 12L14.83 11H15.46A3.5 3.5 0 1 0 11.96 7.5H13.96A1.5 1.5 0 1 1 15.46 9H14L12 11H11V4H9V20H11V13H12L14 15H15.46A1.5 1.5 0 1 1 13.96 16.5H11.96A3.5 3.5 0 1 0 15.46 13Z" />
    </svg>
  }
}
