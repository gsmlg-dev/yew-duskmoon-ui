#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CodeBrackets)]
pub fn r#icon_code_brackets(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15,4V6H18V18H15V20H20V4M4,4V20H9V18H6V6H9V4H4Z" />
    </svg>
  }
}
