#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AudioInputXlr)]
pub fn r#icon_audio_input_xlr(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 4C16.41 4 20 7.59 20 12S16.41 20 12 20 4 16.41 4 12 7.59 4 12 4M12 2C6.5 2 2 6.5 2 12S6.5 22 12 22 22 17.5 22 12 17.5 2 12 2M13.5 16.5C13.5 15.67 12.83 15 12 15S10.5 15.67 10.5 16.5C10.5 17.33 11.17 18 12 18S13.5 17.33 13.5 16.5M9 12C9 11.17 8.33 10.5 7.5 10.5S6 11.17 6 12 6.67 13.5 7.5 13.5 9 12.83 9 12M18 12C18 11.17 17.33 10.5 16.5 10.5C15.67 10.5 15 11.17 15 12S15.67 13.5 16.5 13.5C17.33 13.5 18 12.83 18 12Z" />
    </svg>
  }
}
