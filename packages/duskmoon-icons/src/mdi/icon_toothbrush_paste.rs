#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ToothbrushPaste)]
pub fn r#icon_toothbrush_paste(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17.86 1.5L12.91 6.43L15.03 8.55L16.09 7.5L15.03 6.43L17.86 3.6L18.92 4.66L20 3.6M21.4 3.6L15.56 9.44L13.43 10.15L3 20.57L4.43 22L10.08 16.33L14.33 20.57L18.57 16.33L14.33 12.09L14.5 11.9L16.63 11.19L21.4 6.43C22.18 5.65 22.18 4.38 21.4 3.6M6.55 4.31L2.31 8.55L7.26 13.5L11.5 9.26M18.22 18.1L16.09 20.22L17.5 21.63L19.63 19.5Z" />
    </svg>
  }
}
