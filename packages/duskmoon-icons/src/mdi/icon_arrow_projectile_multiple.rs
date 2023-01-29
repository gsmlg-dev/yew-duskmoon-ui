#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ArrowProjectileMultiple)]
pub fn r#icon_arrow_projectile_multiple(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 16L22 19L20 20L19 22L16 19V17.06L12 13.06L8 17.06V19L5 22L4 20L2 19L5 16H6.94L10.94 12L4.97 6.03L4 7L2 2L7 4L6.03 4.97L12 10.94L17.97 4.97L17 4L22 2L20 7L19.03 6.03L13.06 12L17.06 16H19Z" />
    </svg>
  }
}
