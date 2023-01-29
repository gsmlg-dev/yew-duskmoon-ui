#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowBottomRightThin)]
pub fn r#icon_arrow_bottom_right_thin(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11.93 19L14.76 16.18L5 6.42L6.42 5L16.18 14.76L19 11.94V19Z" />
    </svg>
  }
}
