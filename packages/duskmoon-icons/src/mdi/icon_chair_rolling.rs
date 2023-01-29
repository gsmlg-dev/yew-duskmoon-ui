#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ChairRolling)]
pub fn r#icon_chair_rolling(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 10V13H19V10H22M2 13H5V10H2V13M17 5C17 3.9 16.1 3 15 3H9C7.9 3 7 3.9 7 5V13H17V5M7 15H6V17H11V18L7 22H9.8L12 19.8L14.2 22H17L13 18V17H18V15H7Z" />
    </svg>
  }
}
