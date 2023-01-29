#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FuelCell)]
pub fn r#icon_fuel_cell(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 3V5H4C2.9 5 2 5.9 2 7V17C2 18.1 2.9 19 4 19V20C4 21.1 4.9 22 6 22H7C8.1 22 9 21.1 9 20V19H15V20C15 21.1 15.9 22 17 22H18C19.1 22 20 21.1 20 20V19C21.1 19 22 18.1 22 17H11V5H8V3H6M16 3V5H13V7H22C22 5.9 21.1 5 20 5H18V3H16M7 7V11H9L6 17V13H4L7 7M13 9V11H22V9H13M13 13V15H22V13H13Z" />
    </svg>
  }
}
