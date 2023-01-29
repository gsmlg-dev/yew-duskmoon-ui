#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_AlphaW)]
pub fn r#icon_alpha_w(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9,17A2,2 0 0,1 7,15V7H9V15H11V8H13V15H15V7H17V15A2,2 0 0,1 15,17H9Z" />
    </svg>
  }
}
