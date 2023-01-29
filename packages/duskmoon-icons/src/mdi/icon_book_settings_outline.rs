#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BookSettingsOutline)]
pub fn r#icon_book_settings_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 0H6C4.89 0 4 .895 4 2V18C4 19.11 4.89 20 6 20H18C19.11 20 20 19.11 20 18V2C20 .895 19.11 0 18 0M18 18H6V2H8V10L10.5 7.75L13 10V2H18V18M7 22H9V24H7V22M11 22H13V24H11V22M15 22H17V24H15V22Z" />
    </svg>
  }
}
