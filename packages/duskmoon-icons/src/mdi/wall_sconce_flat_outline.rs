#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_WallSconceFlatOutline)]
pub fn r#icon_wall_sconce_flat_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 5V11H19V5H5M17 9H7V7H17V9M5.27 13.32L3.5 15.09L4.91 16.5L6.68 14.73L5.27 13.32M18.73 13.32L17.32 14.73L19.09 16.5L20.5 15.09L18.73 13.32M11 16V19H13V16H11Z" />
    </svg>
  }
}
