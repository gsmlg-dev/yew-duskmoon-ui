#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_MailboxOpenUpOutline)]
pub fn r#icon_mailbox_open_up_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14,12.85V6.85H18V8.85H16V12.85H14M18,3.85H8A5,5 0 0,0 3,8.85V17.85H1V19.85H21A2,2 0 0,0 23,17.85V8.85A5,5 0 0,0 18,3.85M11,17.85H5V8.85A3,3 0 0,1 8,5.85A3,3 0 0,1 11,8.85V17.85M21,17.85H13V8.85C13,7.76 12.65,6.71 12,5.85H18A3,3 0 0,1 21,8.85V17.85Z" />
    </svg>
  }
}
