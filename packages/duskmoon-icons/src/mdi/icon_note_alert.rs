#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_NoteAlert)]
pub fn r#icon_note_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 3C1.89 3 1 3.89 1 5V19C1 20.11 1.9 21 3 21H17C18.11 21 19 20.11 19 19V9L13 3H3M12 10V4.5L17.5 10H12M23 7V13H21V7H23M21 15H23V17H21V15Z" />
    </svg>
  }
}
