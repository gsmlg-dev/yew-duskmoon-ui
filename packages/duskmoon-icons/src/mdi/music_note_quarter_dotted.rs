#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_MusicNoteQuarterDotted)]
pub fn r#icon_music_note_quarter_dotted(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 13V13.56A3.96 3.96 0 0 0 10 13A4 4 0 1 0 14 17V3H12M16.5 17A1.5 1.5 0 1 1 15 18.5A1.5 1.5 0 0 1 16.5 17Z" />
    </svg>
  }
}
