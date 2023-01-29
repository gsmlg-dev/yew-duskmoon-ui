#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FloorLampTorchiere)]
pub fn r#icon_floor_lamp_torchiere(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7 2L10 9H14L17 2H7M13 20H16V22H8V20H11V10H13V20Z" />
    </svg>
  }
}
