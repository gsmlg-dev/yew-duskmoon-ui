#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CashMarker)]
pub fn r#icon_cash_marker(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.5,16.8C17.8,16.8 17.3,16.2 17.3,15.6C17.3,14.9 17.9,14.4 18.5,14.4C19.1,14.4 19.7,15 19.7,15.6C19.8,16.2 19.2,16.8 18.5,16.8M18.5,12C16.6,12 15,13.6 15,15.5C15,18.1 18.5,22 18.5,22C18.5,22 22,18.1 22,15.5C22,13.6 20.4,12 18.5,12M14.9,11.3C14.6,10 13.4,9 12,9C10.3,9 9,10.3 9,12C9,13.7 10.3,15 12,15C12.4,15 12.7,14.9 13,14.8C13.2,13.4 13.9,12.2 14.9,11.3M13,16H7A2,2 0 0,0 5,14V10A2,2 0 0,0 7,8H17A2,2 0 0,0 19,10C19,10 20,10 21,10.6V6H3V18H13.5C13.3,17.3 13.1,16.7 13,16Z" />
    </svg>
  }
}
