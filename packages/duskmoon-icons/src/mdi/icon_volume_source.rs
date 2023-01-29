#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_VolumeSource)]
pub fn r#icon_volume_source(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 9V15H7L12 20V4L7 9H3M16 15H14V9H16V15M20 19H18V5H20V19Z" />
    </svg>
  }
}
