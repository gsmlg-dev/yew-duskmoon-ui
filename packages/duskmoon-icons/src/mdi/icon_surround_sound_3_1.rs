#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SurroundSound31)]
pub fn r#icon_surround_sound_3_1(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 17H12V15H14V17M20 7V17H18V9H16V7H20M10 15C10 16.1 9.1 17 8 17H4V15H8V13H6V11H8V9H4V7H8C9.1 7 10 7.9 10 9V10.5C10 11.3 9.3 12 8.5 12C9.3 12 10 12.7 10 13.5V15" />
    </svg>
  }
}
