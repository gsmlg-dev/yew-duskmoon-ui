#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ArchiveStarOutline)]
pub fn r#icon_archive_star_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 13.09V10H20V13.09C19.67 13.04 19.34 13 19 13C18.66 13 18.33 13.04 18 13.09M9.5 11C9.22 11 9 11.22 9 11.5V13H15V11.5C15 11.22 14.78 11 14.5 11H9.5M6 19V10H4V21H13.35C13.13 20.37 13 19.7 13 19H6M21 9H3V3H21V9M19 5H5V7H19V5M23 17.89L20.11 17.64L19 15L17.87 17.64L15 17.89L17.18 19.77L16.5 22.58L19 21.09L21.45 22.58L20.8 19.77L23 17.89Z" />
    </svg>
  }
}
