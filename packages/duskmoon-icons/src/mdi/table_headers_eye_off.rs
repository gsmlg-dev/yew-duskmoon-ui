#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_TableHeadersEyeOff)]
pub fn r#icon_table_headers_eye_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2.38 1.73L1.11 3L2.26 4.15A2 2 0 0 0 1 6V19A2 2 0 0 0 3 21H9V11H9.11L12.72 14.61A6.21 6.21 0 0 0 11 17A6.45 6.45 0 0 0 17 21A6.55 6.55 0 0 0 18.84 20.73L20.84 22.73L22.11 21.46L20.58 19.93M7 19H3V16H7M7 14H3V11H7M7 9H3V6H4.11L7 8.89M17 19.5A2.5 2.5 0 0 1 14.56 16.45L17.56 19.45A2.5 2.5 0 0 1 17 19.5M16.24 13L17.85 14.61A2.5 2.5 0 0 1 19.35 16.11L21.94 18.7A6.44 6.44 0 0 0 23 17A6.45 6.45 0 0 0 17 13H16.24M13 6V9H12.2L14.2 11H21V6A2 2 0 0 0 19 4H7.2L9.2 6M15 6H19V9H15Z" />
    </svg>
  }
}