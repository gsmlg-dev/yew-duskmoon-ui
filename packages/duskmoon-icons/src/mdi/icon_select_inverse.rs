#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SelectInverse)]
pub fn r#icon_select_inverse(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5,3H7V5H9V3H11V5H13V3H15V5H17V3H19V5H21V7H19V9H21V11H19V13H21V15H19V17H21V19H19V21H17V19H15V21H13V19H11V21H9V19H7V21H5V19H3V17H5V15H3V13H5V11H3V9H5V7H3V5H5V3Z" />
    </svg>
  }
}
