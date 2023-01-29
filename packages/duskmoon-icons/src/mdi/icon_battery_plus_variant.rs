#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BatteryPlusVariant)]
pub fn r#icon_battery_plus_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.67,4C17.4,4 18,4.6 18,5.33V20.67A1.33,1.33 0 0,1 16.67,22H7.33C6.6,22 6,21.4 6,20.67V5.33A1.33,1.33 0 0,1 7.33,4H9V2H15V4H16.67M16,14V12H13V9H11V12H8V14H11V17H13V14H16Z" />
    </svg>
  }
}
