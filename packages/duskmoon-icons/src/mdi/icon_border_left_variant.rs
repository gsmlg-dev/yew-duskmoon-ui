#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BorderLeftVariant)]
pub fn r#icon_border_left_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15,5H17V3H15M19,21H21V19H19M19,13H21V11H19M19,5H21V3H19M19,17H21V15H19M15,21H17V19H15M19,9H21V7H19M3,21H5V3H3M7,5H9V3H7M7,21H9V19H7M11,5H13V3H11M11,21H13V19H11V21Z" />
    </svg>
  }
}
