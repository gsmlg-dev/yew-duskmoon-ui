#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Stool)]
pub fn r#icon_stool(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 6C20 3.79 18.21 2 16 2H8C5.78 2 4 3.79 4 6V8H7L4 22H6L7.5 15H11V22H13V15H16.5L18 22H20L17 8H20V6M7.93 13L9 8H11V13H7.93M16.07 13H13V8H15L16.07 13Z" />
    </svg>
  }
}
