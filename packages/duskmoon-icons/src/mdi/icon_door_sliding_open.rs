#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DoorSlidingOpen)]
pub fn r#icon_door_sliding_open(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 11V13H4V11H6M22 5H17V19H22V5M7 5H2L2 19H7V5M22 3C23.11 3 24 3.89 24 5V21H0V5C0 3.89 .894 3 2 3H9V19H15V3H22M20 11H18V13H20V11Z" />
    </svg>
  }
}
