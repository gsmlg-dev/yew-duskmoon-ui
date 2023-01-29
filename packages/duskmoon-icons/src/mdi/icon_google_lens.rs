#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_GoogleLens)]
pub fn r#icon_google_lens(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6,2H18A4,4 0 0,1 22,6V12H20V6A2,2 0 0,0 18,4H6A2,2 0 0,0 4,6V18A2,2 0 0,0 6,20H12V22H6A4,4 0 0,1 2,18V6A4,4 0 0,1 6,2M12,8A4,4 0 0,1 16,12A4,4 0 0,1 12,16A4,4 0 0,1 8,12A4,4 0 0,1 12,8M18,16A2,2 0 0,1 20,18A2,2 0 0,1 18,20A2,2 0 0,1 16,18A2,2 0 0,1 18,16Z" />
    </svg>
  }
}
