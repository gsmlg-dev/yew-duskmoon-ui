#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SilverwareClean)]
pub fn r#icon_silverware_clean(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 0L14.38 1.37L13 2L14.38 2.63L15 4L15.63 2.63L17 2L15.63 1.37L15 0M10.5 2L9.41 4.41L7 5.5L9.41 6.59L10.5 9L11.6 6.59L14 5.5L11.6 4.41L10.5 2M18.89 5.14C17.56 5.06 16.04 5.65 14.84 6.84C13.25 8.43 12.75 10.58 13.46 12.11L3.7 21.87L5.11 23.28L12 16.41L18.88 23.29L20.29 21.88L13.41 15L14.88 13.53C16.41 14.24 18.56 13.74 20.15 12.15C22.06 10.24 22.43 7.5 20.96 6.03C20.41 5.5 19.68 5.19 18.89 5.14M3.91 5.5C2.35 7.06 2.35 9.59 3.91 11.16L8.1 15.34L10.93 12.5L3.91 5.5Z" />
    </svg>
  }
}
