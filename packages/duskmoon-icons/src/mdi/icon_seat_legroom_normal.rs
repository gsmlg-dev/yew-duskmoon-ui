#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SeatLegroomNormal)]
pub fn r#icon_seat_legroom_normal(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5,12V3H3V12A5,5 0 0,0 8,17H14V15H8A3,3 0 0,1 5,12M20.5,18H19V11A2,2 0 0,0 17,9H12V3H6V11A3,3 0 0,0 9,14H16V21H20.5A1.5,1.5 0 0,0 22,19.5A1.5,1.5 0 0,0 20.5,18Z" />
    </svg>
  }
}
