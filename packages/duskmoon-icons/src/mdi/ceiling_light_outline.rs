#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CeilingLightOutline)]
pub fn r#icon_ceiling_light_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.76 11L16.76 15H7.24L9.24 11H14.76M13 4H11V9H8L4 17H20L16 9H13V4M14 18H10C10 19.11 10.9 20 12 20S14 19.11 14 18Z" />
    </svg>
  }
}
