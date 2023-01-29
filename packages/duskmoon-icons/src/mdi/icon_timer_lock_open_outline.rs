#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TimerLockOpenOutline)]
pub fn r#icon_timer_lock_open_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 8H13V14H11V8M13 19.92C12.67 19.97 12.34 20 12 20C8.13 20 5 16.87 5 13S8.13 6 12 6C14.44 6 16.59 7.26 17.85 9.15C18.22 9.06 18.6 9 19 9C19.39 9 19.76 9.06 20.12 9.14C19.82 8.5 19.46 7.93 19.03 7.39L20.45 5.97C20 5.46 19.55 5 19.04 4.56L17.62 6C16.07 4.74 14.12 4 12 4C7.03 4 3 8.03 3 13S7.03 22 12 22C12.42 22 12.83 21.96 13.24 21.91C13.09 21.53 13 21.12 13 20.7V19.92M15 1H9V3H15V1M21.8 16H17.5V13.5C17.5 12.7 18.2 12.2 19 12.2S20.5 12.7 20.5 13.5V14H21.8V13.5C21.8 12.1 20.4 11 19 11S16.2 12.1 16.2 13.5V16C15.6 16 15 16.6 15 17.2V20.7C15 21.4 15.6 22 16.2 22H21.7C22.4 22 23 21.4 23 20.8V17.3C23 16.6 22.4 16 21.8 16Z" />
    </svg>
  }
}
