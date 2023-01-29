#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CarCoolantLevel)]
pub fn r#icon_car_coolant_level(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 7V5H21V3H13V5H15V7H1V21H23V7H19M21 14.42C20.71 14.13 20.5 14 20 14C19.33 14 19.21 14.21 18.71 14.71S17.33 16 16 16 13.79 15.21 13.29 14.71 12.67 14 12 14C11.78 14 11.63 14.03 11.5 14.08C11.22 14.18 11.05 14.37 10.71 14.71C10.21 15.21 9.33 16 8 16S5.79 15.21 5.29 14.71 4.67 14 4 14C3.5 14 3.29 14.13 3 14.42V9H21V14.42Z" />
    </svg>
  }
}
