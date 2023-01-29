#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CastEducation)]
pub fn r#icon_cast_education(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21,3H3A2,2 0 0,0 1,5V8H3V5H21V19H14V21H21A2,2 0 0,0 23,19V5A2,2 0 0,0 21,3M1,18V21H4A3,3 0 0,0 1,18M1,14V16A5,5 0 0,1 6,21H8A7,7 0 0,0 1,14M1,10V12A9,9 0 0,1 10,21H12C12,14.92 7.07,10 1,10M11,11.09V13.09L14.5,15L18,13.09V11.09L14.5,13L11,11.09M14.5,6L9,9L14.5,12L20,9L14.5,6Z" />
    </svg>
  }
}
