#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FoodOff)]
pub fn r#icon_food_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11.35 8.5L11 5H16V1H18V5H23L21.62 18.79L11.35 8.5M1 21V22C1 22.55 1.45 23 2 23H15C15.55 23 16 22.55 16 22V21H1M21.9 21.9L2.1 2.1L.69 3.5L6.39 9.21C3.28 9.87 1 12 1 15H12.17L14.17 17H1V19H16V18.83L20.5 23.32L21.9 21.9Z" />
    </svg>
  }
}
