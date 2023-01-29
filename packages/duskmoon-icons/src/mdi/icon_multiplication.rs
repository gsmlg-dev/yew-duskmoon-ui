#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Multiplication)]
pub fn r#icon_multiplication(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11,3H13V10.27L19.29,6.64L20.29,8.37L14,12L20.3,15.64L19.3,17.37L13,13.72V21H11V13.73L4.69,17.36L3.69,15.63L10,12L3.72,8.36L4.72,6.63L11,10.26V3Z" />
    </svg>
  }
}
