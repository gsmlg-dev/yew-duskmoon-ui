#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MusicNoteHalf)]
pub fn r#icon_music_note_half(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16 3H14V13.56A3.96 3.96 0 0 0 12 13A4 4 0 1 0 16 17V3M12 19A2 2 0 1 1 14 17A2 2 0 0 1 12 19Z" />
    </svg>
  }
}
