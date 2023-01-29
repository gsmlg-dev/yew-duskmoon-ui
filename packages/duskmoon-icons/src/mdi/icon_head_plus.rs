#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_HeadPlus)]
pub fn r#icon_head_plus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 3C9.2 3 6.2 5.9 6 9.7L4.1 12.2C3.9 12.5 4.1 13 4.5 13H6V16C6 17.1 6.9 18 8 18H9V21H16V16.3C18.4 15.2 20 12.8 20 10C20 6.1 16.9 3 13 3M17 10H14V13H12V10H9V8H12V5H14V8H17V10Z" />
    </svg>
  }
}
