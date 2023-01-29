#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TimerSync)]
pub fn r#icon_timer_sync(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 3H9V1H15V3M13 17.5C13 19.08 13.56 20.5 14.5 21.65C13.7 21.87 12.87 22 12 22C7.03 22 3 17.97 3 13S7.03 4 12 4C14.12 4 16.07 4.74 17.62 6L19.04 4.56C19.55 5 20 5.46 20.45 5.97L19.03 7.39C19.89 8.46 20.5 9.74 20.8 11.13C20.38 11.05 19.94 11 19.5 11C15.91 11 13 13.91 13 17.5M13 7H11V14H13V7M19 13.5V12L16.75 14.25L19 16.5V15C20.38 15 21.5 16.12 21.5 17.5C21.5 17.9 21.41 18.28 21.24 18.62L22.33 19.71C22.75 19.08 23 18.32 23 17.5C23 15.29 21.21 13.5 19 13.5M19 20C17.62 20 16.5 18.88 16.5 17.5C16.5 17.1 16.59 16.72 16.76 16.38L15.67 15.29C15.25 15.92 15 16.68 15 17.5C15 19.71 16.79 21.5 19 21.5V23L21.25 20.75L19 18.5V20Z" />
    </svg>
  }
}
