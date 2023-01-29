#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_LedStripVariantOff)]
pub fn r#icon_led_strip_variant_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.11 21.46L2.39 1.73L1.11 3L2.59 4.5L2 6.91L6.03 7.92L9.08 10.97L6.25 12.11L11.55 13.44L12.94 14.83L4.66 12.75L3.71 16.66L18.46 20.35L20.84 22.73L22.11 21.46M7.8 16.65L5.88 16.16L6.35 14.21L8.28 14.69L7.8 16.65M11.65 17.61L9.73 17.13L10.2 15.18L12.13 15.66L11.65 17.61M15.5 18.58L13.58 18.09L14.06 16.14L14.31 16.2L15.74 17.63L15.5 18.58M19.73 16.53L22 17.1L21.67 18.47L19.73 16.53M15.85 12.65L14.2 11L17.75 11.89L15.85 12.65M10.42 5.91L10.16 6.96L12.82 9.62L19.34 11.25L20.29 7.34L7.29 4.09L8.66 5.46L10.42 5.91M16.2 7.35L18.13 7.84L17.65 9.79L15.72 9.31L16.2 7.35M12.35 6.39L14.27 6.87L13.8 8.82L11.87 8.34L12.35 6.39Z" />
    </svg>
  }
}
