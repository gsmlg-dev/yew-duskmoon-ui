#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BatteryArrowDownOutline)]
pub fn r#icon_battery_arrow_down_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13.54 22H7.33C6.6 22 6 21.4 6 20.67V5.33C6 4.6 6.6 4 7.33 4H9V2H15V4H16.67C17.4 4 18 4.6 18 5.33V12C17.3 12 16.63 12.13 16 12.35V6H8V20H12.35C12.61 20.75 13 21.42 13.54 22M14.94 18.5L17.94 21.5L20.94 18.5H18.94V14.5H16.94V18.5H14.94" />
    </svg>
  }
}
