#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_NoteSearchOutline)]
pub fn r#icon_note_search_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 3H5C3.89 3 3 3.89 3 5V10.82C3.6 10.24 4.28 9.8 5 9.5V5H12V10.82C12.03 10.85 12.07 10.87 12.1 10.9C12.44 11.24 12.73 11.61 12.97 12H19V19H12.97C12.73 19.39 12.44 19.76 12.1 20.1C11.74 20.45 11.35 20.74 10.94 21H19C20.11 21 21 20.11 21 19V9L15 3M14 10V4.5L19.5 10H14M7.5 11C5 11 3 13 3 15.5C3 16.38 3.25 17.21 3.69 17.9L.61 21L2 22.39L5.12 19.32C5.81 19.75 6.63 20 7.5 20C10 20 12 18 12 15.5S10 11 7.5 11M7.5 18C6.12 18 5 16.88 5 15.5S6.12 13 7.5 13 10 14.12 10 15.5 8.88 18 7.5 18Z" />
    </svg>
  }
}
