#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_WallSconce)]
pub fn r#icon_wall_sconce(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11,4L7,13H19L15,4H11M4,14V22H6V19H14V14H12V17H6V14H4Z" />
    </svg>
  }
}
