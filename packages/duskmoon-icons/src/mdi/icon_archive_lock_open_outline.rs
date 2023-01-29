#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ArchiveLockOpenOutline)]
pub fn r#icon_archive_lock_open_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.5 11C14.67 11 14.82 11.09 14.91 11.22C14.56 11.76 14.34 12.36 14.26 13H9V11.5C9 11.22 9.22 11 9.5 11H14.5M13 19H6V10H4V21H13.03C13 20.9 13 20.8 13 20.7V19M21 9H3V3H21V9M19 5H5V7H19V5M21.8 16H17.5V13.5C17.5 12.7 18.2 12.2 19 12.2S20.5 12.7 20.5 13.5V14H21.8V13.5C21.8 12.1 20.4 11 19 11S16.2 12.1 16.2 13.5V16C15.6 16 15 16.6 15 17.2V20.7C15 21.4 15.6 22 16.2 22H21.7C22.4 22 23 21.4 23 20.8V17.3C23 16.6 22.4 16 21.8 16Z" />
    </svg>
  }
}
