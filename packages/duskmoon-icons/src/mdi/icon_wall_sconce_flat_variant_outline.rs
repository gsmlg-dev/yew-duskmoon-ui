#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_WallSconceFlatVariantOutline)]
pub fn r#icon_wall_sconce_flat_variant_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 13V19H19V13H5M17 17H7V15H17V17M18.73 10.68L20.5 8.91L19.09 7.5L17.32 9.27L18.73 10.68M5.27 10.68L6.68 9.27L4.91 7.5L3.5 8.91L5.27 10.68M13 8V5H11V8H13Z" />
    </svg>
  }
}
