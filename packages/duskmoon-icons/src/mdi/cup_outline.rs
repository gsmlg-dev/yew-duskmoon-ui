#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CupOutline)]
pub fn r#icon_cup_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 2L5 20.23C5.13 21.23 5.97 22 7 22H17C18 22 18.87 21.23 19 20.23L21 2H3M5.22 4H18.78L17 20H7L5.22 4Z" />
    </svg>
  }
}
