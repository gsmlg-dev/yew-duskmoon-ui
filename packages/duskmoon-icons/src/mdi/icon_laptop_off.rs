#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_LaptopOff)]
pub fn r#icon_laptop_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M1,4.27L2.28,3L20,20.72L18.73,22L16.73,20H0V18H4C2.89,18 2,17.1 2,16V6C2,5.78 2.04,5.57 2.1,5.37L1,4.27M4,16H12.73L4,7.27V16M20,16V6H7.82L5.82,4H20A2,2 0 0,1 22,6V16A2,2 0 0,1 20,18H24V20H21.82L17.82,16H20Z" />
    </svg>
  }
}
