#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ImageAreaClose)]
pub fn r#icon_image_area_close(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,23L8,19H16L12,23M20,3A2,2 0 0,1 22,5V15A2,2 0 0,1 20,17H4A2,2 0 0,1 2,15V5A2,2 0 0,1 4,3H20M5,14H19L14.5,8L11,12.5L8.5,9.5L5,14Z" />
    </svg>
  }
}
