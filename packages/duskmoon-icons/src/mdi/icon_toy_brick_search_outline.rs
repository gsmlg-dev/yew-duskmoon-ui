#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ToyBrickSearchOutline)]
pub fn r#icon_toy_brick_search_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 6V5A2 2 0 0 0 17 3H15A2 2 0 0 0 13 5V6H11V5A2 2 0 0 0 9 3H7A2 2 0 0 0 5 5V6H3V20H11.81A6.59 6.59 0 0 1 10.5 18H5V8H19V9.5A6.59 6.59 0 0 1 21 10.81V6M20.31 17.9A4.5 4.5 0 1 0 18.88 19.32L22 22.39L23.39 21M16.5 18A2.5 2.5 0 1 1 19 15.5A2.5 2.5 0 0 1 16.5 18Z" />
    </svg>
  }
}
