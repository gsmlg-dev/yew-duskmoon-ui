#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SeatFlat)]
pub fn r#icon_seat_flat(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22,11V13H9V7H18A4,4 0 0,1 22,11M2,14V16H8V18H16V16H22V14M7.14,12.1C8.3,10.91 8.28,9 7.1,7.86C5.91,6.7 4,6.72 2.86,7.9C1.7,9.09 1.72,11 2.9,12.14C4.09,13.3 6,13.28 7.14,12.1Z" />
    </svg>
  }
}
