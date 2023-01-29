#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_SyllabaryKatakanaHalfwidth)]
pub fn r#icon_syllabary_katakana_halfwidth(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8 4V6H14L13 10.81L15 11.2L16 6.1V4M10 9V12C10 14.86 9.34 17.29 8.08 18.61L9.5 20C11.35 18.08 12 15.15 12 12V9Z" />
    </svg>
  }
}
