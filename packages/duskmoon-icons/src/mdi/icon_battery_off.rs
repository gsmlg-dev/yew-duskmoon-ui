#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BatteryOff)]
pub fn r#icon_battery_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 14.8L7.21 4H9V2H15V4H16.67A1.34 1.34 0 0 1 18 5.33M18 17.35L3.38 2.73L2.11 4L6 7.89V20.67A1.34 1.34 0 0 0 7.33 22H16.67A1.34 1.34 0 0 0 18 20.67V19.89L20.84 22.73L22.11 21.46Z" />
    </svg>
  }
}
