#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ToyBrickMinus)]
pub fn r#icon_toy_brick_minus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M23 20H15V18H23V20M13 19C13 15.69 15.69 13 19 13C19.7 13 20.37 13.13 21 13.35V6H19V5C19 3.9 18.11 3 17 3H15C13.9 3 13 3.9 13 5V6H11V5C11 3.9 10.11 3 9 3H7C5.9 3 5 3.9 5 5V6H3V20H13.09C13.04 19.67 13 19.34 13 19Z" />
    </svg>
  }
}
