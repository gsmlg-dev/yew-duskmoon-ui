#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ProjectorScreenVariantOff)]
pub fn r#icon_projector_screen_variant_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.84 22.73L16.11 18H5V9H4C3.45 9 3 8.55 3 8V7C3 6.45 3.45 6 4 6H4.11L1.11 3L2.39 1.73L22.11 21.46L20.84 22.73M19 9H20C20.55 9 21 8.55 21 8V7C21 6.45 20.55 6 20 6H9.2L19 15.8V9Z" />
    </svg>
  }
}
