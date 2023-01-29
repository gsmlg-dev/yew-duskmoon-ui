#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ClockTimeTwelveOutline)]
pub fn r#icon_clock_time_twelve_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 20C16.42 20 20 16.42 20 12S16.42 4 12 4 4 7.58 4 12 7.58 20 12 20M12 2C17.5 2 22 6.5 22 12S17.5 22 12 22C6.47 22 2 17.5 2 12C2 6.5 6.5 2 12 2M12.5 13.03H11V7H12.5V13.03Z" />
    </svg>
  }
}
