#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TimerRemoveOutline)]
pub fn r#icon_timer_remove_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13.08 19.91C13.18 20.6 13.4 21.25 13.71 21.83C13.16 21.94 12.59 22 12 22C7.03 22 3 17.97 3 13S7.03 4 12 4C14.12 4 16.07 4.74 17.62 6L19.04 4.56C19.55 5 20 5.46 20.45 5.97L19.03 7.39C20.26 8.93 21 10.88 21 13C21 13.12 21 13.23 21 13.35C20.36 13.13 19.7 13 19 13C19 9.13 15.87 6 12 6S5 9.13 5 13 8.13 20 12 20C12.37 20 12.72 19.96 13.08 19.91M11 14H13V8H11V14M15 1H9V3H15V1M22.54 16.88L21.12 15.47L19 17.59L16.88 15.47L15.47 16.88L17.59 19L15.47 21.12L16.88 22.54L19 20.41L21.12 22.54L22.54 21.12L20.41 19L22.54 16.88Z" />
    </svg>
  }
}
