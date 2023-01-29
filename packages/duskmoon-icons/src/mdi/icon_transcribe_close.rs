#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TranscribeClose)]
pub fn r#icon_transcribe_close(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,23L8,19H16L12,23M20,3A2,2 0 0,1 22,5V15A2,2 0 0,1 20,17H4A2,2 0 0,1 2,15V5A2,2 0 0,1 4,3H20M18,15V13H12.5L10.5,15H18M6,15H8.5L15.35,8.12C15.55,7.93 15.55,7.61 15.35,7.42L13.59,5.65C13.39,5.45 13.07,5.45 12.88,5.65L6,12.53V15Z" />
    </svg>
  }
}
