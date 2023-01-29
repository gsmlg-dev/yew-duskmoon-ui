#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ArchiveSettingsOutline)]
pub fn r#icon_archive_settings_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 2V8H21V2H3M19 6H5V4H19V6M18 9H20V20H4V9H6V18H18V9M15 10.5V12H9V10.5C9 10.22 9.22 10 9.5 10H14.5C14.78 10 15 10.22 15 10.5M7 22H9V24H7V22M11 22H13V24H11V22M15 22H17V24H15V22Z" />
    </svg>
  }
}
