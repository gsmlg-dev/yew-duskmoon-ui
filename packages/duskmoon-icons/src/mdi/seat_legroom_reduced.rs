#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_SeatLegroomReduced)]
pub fn r#icon_seat_legroom_reduced(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.97,19.2C20.15,20.16 19.42,21 18.5,21H14V18L15,14H9A3,3 0 0,1 6,11V3H12V9H17A2,2 0 0,1 19,11L17,18H18.44C19.17,18 19.83,18.5 19.97,19.2M5,12V3H3V12A5,5 0 0,0 8,17H12V15H8A3,3 0 0,1 5,12Z" />
    </svg>
  }
}
