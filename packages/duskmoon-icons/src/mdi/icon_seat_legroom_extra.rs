#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SeatLegroomExtra)]
pub fn r#icon_seat_legroom_extra(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4,12V3H2V12A5,5 0 0,0 7,17H13V15H7A3,3 0 0,1 4,12M22.83,17.24C22.45,16.5 21.54,16.27 20.8,16.61L19.71,17.11L16.3,10.13C15.96,9.45 15.27,9 14.5,9H11V3H5V11A3,3 0 0,0 8,14H15L18.41,21L22.13,19.3C22.9,18.94 23.23,18 22.83,17.24Z" />
    </svg>
  }
}
