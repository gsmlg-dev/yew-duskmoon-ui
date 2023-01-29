#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_AutoDownload)]
pub fn r#icon_auto_download(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 17V19H11V17H22M19 4.5V9.5H22L16.5 15L11 9.5H14V4.5H19M10.7 15H8.8L8.1 13H4.9L4.2 15H2.3L5.5 6H7.5L10.7 15M7.65 11.65L6.5 8L5.35 11.65H7.65Z" />
    </svg>
  }
}
