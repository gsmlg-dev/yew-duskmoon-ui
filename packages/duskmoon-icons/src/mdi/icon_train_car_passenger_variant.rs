#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TrainCarPassengerVariant)]
pub fn r#icon_train_car_passenger_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 7H3C1.9 7 1 7.9 1 9V17H2C2 18.11 2.9 19 4 19S6 18.11 6 17H18C18 18.11 18.9 19 20 19S22 18.11 22 17H23V9C23 7.9 22.11 7 21 7M11 12H3V9H11V12M21 12H13V9H21V12Z" />
    </svg>
  }
}
