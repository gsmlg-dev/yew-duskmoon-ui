#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_HandFrontRight)]
pub fn r#icon_hand_front_right(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.75 8C19.06 8 18.5 8.56 18.5 9.25V15H18C16.35 15 15 16.35 15 18H14C14 15.96 15.53 14.28 17.5 14.03V3.25C17.5 2.56 16.94 2 16.25 2C15.56 2 15 2.56 15 3.25V11H14V1.25C14 .56 13.44 0 12.75 0S11.5 .56 11.5 1.25V11H10.5V2.75C10.5 2.06 9.94 1.5 9.25 1.5S8 2.06 8 2.75V12H7V5.75C7 5.06 6.44 4.5 5.75 4.5S4.5 5.06 4.5 5.75V15.75C4.5 20.31 8.19 24 12.75 24S21 20.31 21 15.75V9.25C21 8.56 20.44 8 19.75 8Z" />
    </svg>
  }
}
