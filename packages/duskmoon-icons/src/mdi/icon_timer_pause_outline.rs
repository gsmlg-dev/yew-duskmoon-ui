#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TimerPauseOutline)]
pub fn r#icon_timer_pause_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 13.35C20.36 13.13 19.7 13 19 13C19 9.13 15.87 6 12 6S5 9.13 5 13 8.13 20 12 20C12.37 20 12.72 19.96 13.08 19.91C13.18 20.6 13.4 21.25 13.71 21.83C13.16 21.94 12.59 22 12 22C7.03 22 3 17.97 3 13S7.03 4 12 4C14.12 4 16.07 4.74 17.62 6L19.04 4.56C19.55 5 20 5.46 20.45 5.97L19.03 7.39C20.26 8.93 21 10.88 21 13C21 13.12 21 13.23 21 13.35M11 14H13V8H11V14M15 1H9V3H15V1M19.63 16.5V21.5H21.5V16.5H19.63M16.5 21.5H18.38V16.5H16.5V21.5Z" />
    </svg>
  }
}
