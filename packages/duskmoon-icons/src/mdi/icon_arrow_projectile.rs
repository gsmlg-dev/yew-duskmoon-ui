#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ArrowProjectile)]
pub fn r#icon_arrow_projectile(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 2L20 7L19.03 6.03L8 17.06V19L5 22L4 20L2 19L5 16H6.94L17.97 4.97L17 4L22 2Z" />
    </svg>
  }
}
