#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ThermometerProbeOff)]
pub fn r#icon_thermometer_probe_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2 3.64L9.24 10.88L2.81 17.31C1.74 18.38 1.74 20.12 2.81 21.2C3.88 22.27 5.62 22.27 6.7 21.2L13.13 14.77L20.73 22.37L22 21.1L3.27 2.37L2 3.64M10.23 11.86L12.14 13.77C11.89 14.14 11.47 14.38 11 14.38C10.24 14.38 9.62 13.76 9.62 13C9.62 12.53 9.86 12.11 10.23 11.86M14.46 11L13 9.55L13.47 9.47L19.47 3.47L22 2L20.54 4.54L14.54 10.54L14.46 11Z" />
    </svg>
  }
}
