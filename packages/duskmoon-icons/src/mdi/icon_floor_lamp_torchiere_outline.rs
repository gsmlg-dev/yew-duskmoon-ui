#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FloorLampTorchiereOutline)]
pub fn r#icon_floor_lamp_torchiere_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 4L12.7 7H11.3L10 4H14M17 2H7L10 9H14L17 2M13 20H16V22H8V20H11V10H13V20Z" />
    </svg>
  }
}
