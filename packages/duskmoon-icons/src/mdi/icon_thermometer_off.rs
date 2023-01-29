#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ThermometerOff)]
pub fn r#icon_thermometer_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 7.8L9 5.8V5C9 3.34 10.34 2 12 2S15 3.34 15 5V11.8L11.2 8H13V5C13 4.45 12.55 4 12 4S11 4.45 11 5V7.8M22.11 21.46L2.39 1.73L1.11 3L9 10.89V13C6.79 14.66 6.34 17.79 8 20C9.66 22.21 12.79 22.66 15 21C15.84 20.37 16.4 19.5 16.71 18.6L20.84 22.73L22.11 21.46Z" />
    </svg>
  }
}
