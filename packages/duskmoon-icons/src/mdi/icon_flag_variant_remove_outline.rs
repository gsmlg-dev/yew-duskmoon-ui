#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FlagVariantRemoveOutline)]
pub fn r#icon_flag_variant_remove_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 3C6.6 3 7 3.4 7 4V4.9C8.1 4.4 9.5 4 11 4C14 4 14 6 16 6C19 6 20 4 20 4V12C20 12 19 14 16 14S13 12 11 12C8 12 7 14 7 14V21H5V4C5 3.4 5.4 3 6 3M7 7.2V11.4C7 11.4 9 9.9 11 9.9S14 11.9 16 11.9 18 10.9 18 10.9V7.5C18 7.5 17 8 16 8C14 8 13 6 11 6S7 7.2 7 7.2M21.1 15.5L19 17.6L16.9 15.5L15.5 16.9L17.6 19L15.5 21.1L16.9 22.5L19 20.4L21.1 22.5L22.5 21.1L20.4 19L22.5 16.9L21.1 15.5Z" />
    </svg>
  }
}
