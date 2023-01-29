#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DesktopTower)]
pub fn r#icon_desktop_tower(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8,2H16A2,2 0 0,1 18,4V20A2,2 0 0,1 16,22H8A2,2 0 0,1 6,20V4A2,2 0 0,1 8,2M8,4V6H16V4H8M16,8H8V10H16V8M16,18H14V20H16V18Z" />
    </svg>
  }
}
