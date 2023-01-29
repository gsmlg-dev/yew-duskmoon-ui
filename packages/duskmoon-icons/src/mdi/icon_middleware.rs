#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Middleware)]
pub fn r#icon_middleware(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M23 12L19 8V11H14.82C14.4 9.84 13.3 9 12 9S9.6 9.84 9.18 11H5L2 8V16L5 13H9.18C9.6 14.16 10.7 15 12 15S14.4 14.16 14.82 13H19V16L23 12Z" />
    </svg>
  }
}
