#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_SurroundSound71)]
pub fn r#icon_surround_sound_7_1(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 17H12V15H14V17M20 7V17H18V9H16V7H20M4 17L8 9H4V7H10V9L6 17" />
    </svg>
  }
}
