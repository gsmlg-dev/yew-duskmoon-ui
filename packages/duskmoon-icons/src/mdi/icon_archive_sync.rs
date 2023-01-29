#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArchiveSync)]
pub fn r#icon_archive_sync(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 7H3V3H21V7M9.5 11H14.5C14.78 11 15 11.22 15 11.5V12.82C16.17 11.69 17.75 11 19.5 11C19.67 11 19.84 11 20 11.03V8H4V21H14.03C13.38 20 13 18.79 13 17.5C13 15.75 13.69 14.17 14.82 13H9V11.5C9 11.22 9.22 11 9.5 11M19 13.5V12L16.75 14.25L19 16.5V15C20.38 15 21.5 16.12 21.5 17.5C21.5 17.9 21.41 18.28 21.24 18.62L22.33 19.71C22.75 19.08 23 18.32 23 17.5C23 15.29 21.21 13.5 19 13.5M19 20C17.62 20 16.5 18.88 16.5 17.5C16.5 17.1 16.59 16.72 16.76 16.38L15.67 15.29C15.25 15.92 15 16.68 15 17.5C15 19.71 16.79 21.5 19 21.5V23L21.25 20.75L19 18.5V20Z" />
    </svg>
  }
}
