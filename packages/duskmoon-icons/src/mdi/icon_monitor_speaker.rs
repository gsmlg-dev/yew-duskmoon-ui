#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MonitorSpeaker)]
pub fn r#icon_monitor_speaker(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 2C1.9 2 1 2.9 1 4V16C1 17.1 1.9 18 3 18H10V20H8V22H12V16H3V4H21V7H23V4C23 2.9 22.1 2 21 2H3M21 9H16C14.9 9 14 9.9 14 11V20C14 21.1 14.9 22 16 22H21C22.1 22 23 21.1 23 20V11C23 9.9 22.1 9 21 9M18.5 10.5C19.3 10.5 20 11.2 20 12S19.3 13.5 18.5 13.5 17 12.8 17 12 17.7 10.5 18.5 10.5M18.5 20.5C16.8 20.5 15.5 19.2 15.5 17.5C15.5 15.9 16.8 14.5 18.4 14.5H18.5C20.2 14.5 21.5 15.8 21.5 17.5S20.2 20.5 18.5 20.5M18.5 16C17.7 16 17 16.7 17 17.5S17.7 19 18.5 19 20 18.3 20 17.5 19.3 16 18.5 16Z" />
    </svg>
  }
}
