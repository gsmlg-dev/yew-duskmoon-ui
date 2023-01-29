#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Hdr)]
pub fn r#icon_hdr(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21,11.5V10.5C21,9.7 20.3,9 19.5,9H16V15H17.5V13H18.6L19.5,15H21L20.1,12.9C20.6,12.6 21,12.1 21,11.5M19.5,11.5H17.5V10.5H19.5V11.5M6.5,11H4.5V9H3V15H4.5V12.5H6.5V15H8V9H6.5V11M13,9H9.5V15H13C13.8,15 14.5,14.3 14.5,13.5V10.5C14.5,9.7 13.8,9 13,9M13,13.5H11V10.5H13V13.5Z" />
    </svg>
  }
}
