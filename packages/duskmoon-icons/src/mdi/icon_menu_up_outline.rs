#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MenuUpOutline)]
pub fn r#icon_menu_up_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18,16V14.5L12,8.5L6,14.5V16H18M12,11.33L14.67,14H9.33L12,11.33Z" />
    </svg>
  }
}
