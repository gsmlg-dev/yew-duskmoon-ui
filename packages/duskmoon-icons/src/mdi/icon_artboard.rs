#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Artboard)]
pub fn r#icon_artboard(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17 9V15H7V9H17M19 3H17V6H19V3M7 3H5V6H7V3M23 7H20V9H23V7M19 7H5V17H19V7M4 7H1V9H4V7M23 15H20V17H23V15M4 15H1V17H4V15M19 18H17V21H19V18M7 18H5V21H7V18Z" />
    </svg>
  }
}
