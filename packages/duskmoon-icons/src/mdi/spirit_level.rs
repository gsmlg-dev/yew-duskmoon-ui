#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_SpiritLevel)]
pub fn r#icon_spirit_level(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 8H2V16H22V8M18 14V10H20V14H18M11 12H13C14.1 12 15 11.11 15 10H17V14H7V10H9C9 11.11 9.9 12 11 12M4 14V10H6V14H4Z" />
    </svg>
  }
}
