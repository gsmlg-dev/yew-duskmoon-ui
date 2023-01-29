#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TimerMarkerOutline)]
pub fn r#icon_timer_marker_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 8H13V14H11V8M15 1H9V3H15V1M12 20C8.13 20 5 16.87 5 13S8.13 6 12 6C14.79 6 17.2 7.64 18.32 10C18.38 10 18.44 10 18.5 10C19.25 10 19.97 10.16 20.62 10.44C20.29 9.32 19.74 8.28 19.03 7.39L20.45 5.97C20 5.46 19.55 5 19.04 4.56L17.62 6C16.07 4.74 14.12 4 12 4C7.03 4 3 8.03 3 13S7.03 22 12 22C13.21 22 14.37 21.76 15.42 21.33C15.05 20.81 14.67 20.22 14.32 19.6C13.59 19.85 12.81 20 12 20M22 15.5C22 18.1 18.5 22 18.5 22S15 18.1 15 15.5C15 13.6 16.6 12 18.5 12S22 13.6 22 15.5M19.7 15.6C19.7 15 19.1 14.4 18.5 14.4S17.3 14.9 17.3 15.6C17.3 16.2 17.8 16.8 18.5 16.8S19.8 16.2 19.7 15.6Z" />
    </svg>
  }
}
