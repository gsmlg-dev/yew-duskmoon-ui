#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AlphaP)]
pub fn r#icon_alpha_p(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9,7V17H11V13H13A2,2 0 0,0 15,11V9A2,2 0 0,0 13,7H9M11,9H13V11H11V9Z" />
    </svg>
  }
}
