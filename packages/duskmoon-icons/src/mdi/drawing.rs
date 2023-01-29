#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Drawing)]
pub fn r#icon_drawing(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8.5,3A5.5,5.5 0 0,1 14,8.5C14,9.83 13.53,11.05 12.74,12H21V21H12V12.74C11.05,13.53 9.83,14 8.5,14A5.5,5.5 0 0,1 3,8.5A5.5,5.5 0 0,1 8.5,3Z" />
    </svg>
  }
}
