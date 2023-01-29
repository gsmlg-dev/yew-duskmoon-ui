#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ArchiveOffOutline)]
pub fn r#icon_archive_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8.2 5L6.2 3H21V9H12.2L10.2 7H19V5H8.2M20 16.8V10H18V14.8L20 16.8M20 19.35V19.34L18 17.34V17.35L9.66 9H9.66L7.66 7H7.66L6.13 5.47L2.39 1.73L1.11 3L3 4.89V9H7.11L17.11 19H6V10H4V21H19.11L20.84 22.73L22.11 21.46L20 19.35Z" />
    </svg>
  }
}
