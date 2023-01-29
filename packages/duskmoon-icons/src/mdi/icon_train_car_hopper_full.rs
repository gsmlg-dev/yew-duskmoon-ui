#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TrainCarHopperFull)]
pub fn r#icon_train_car_hopper_full(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 8C19.04 6.19 15.74 5 12 5S4.96 6.19 3 8H1V17H2C2 18.11 2.9 19 4 19S6 18.11 6 17H18C18 18.11 18.9 19 20 19S22 18.11 22 17H23V8H21M13 15V11H11V15H8V11H6V15H3V10H21V15H18V11H16V15H13Z" />
    </svg>
  }
}
