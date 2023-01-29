#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MusicNoteOutline)]
pub fn r#icon_music_note_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 3V13.55A4 4 0 1 0 14 17V7H18V3M10 19A2 2 0 1 1 12 17A2 2 0 0 1 10 19Z" />
    </svg>
  }
}
