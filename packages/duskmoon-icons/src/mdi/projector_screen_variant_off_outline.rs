#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ProjectorScreenVariantOffOutline)]
pub fn r#icon_projector_screen_variant_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.11 21.46L2.39 1.73L1.11 3L4.11 6H4C3.45 6 3 6.45 3 7V8C3 8.55 3.45 9 4 9H5V18H16.11L20.84 22.73L22.11 21.46M7 16V9H7.11L14.11 16H7M12.2 9L9.2 6H20C20.55 6 21 6.45 21 7V8C21 8.55 20.55 9 20 9H19V15.8L17 13.8V9H12.2Z" />
    </svg>
  }
}
