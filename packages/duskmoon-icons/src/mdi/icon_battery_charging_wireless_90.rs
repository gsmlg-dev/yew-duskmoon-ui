#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BatteryChargingWireless90)]
pub fn r#icon_battery_charging_wireless_90(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.07,4.93L18.66,6.34C21.79,9.46 21.79,14.53 18.66,17.66L20.07,19.07C23.97,15.17 23.97,8.84 20.07,4.93M17.24,7.76L15.83,9.17C17.39,10.73 17.39,13.26 15.83,14.83L17.24,16.24C19.58,13.9 19.58,10.1 17.24,7.76M13,4H11V2H5V4H3A1,1 0 0,0 2,5V21A1,1 0 0,0 3,22H13A1,1 0 0,0 14,21V5A1,1 0 0,0 13,4M12,7.5H4V6H12V7.5Z" />
    </svg>
  }
}
