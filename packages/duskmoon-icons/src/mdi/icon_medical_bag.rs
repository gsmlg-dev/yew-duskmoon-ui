#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MedicalBag)]
pub fn r#icon_medical_bag(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,3L8,5V7H5C3.85,7 3.12,8 3,9L2,19C1.88,20 2.54,21 4,21H20C21.46,21 22.12,20 22,19L21,9C20.88,8 20.06,7 19,7H16V5L14,3H10M10,5H14V7H10V5M11,10H13V13H16V15H13V18H11V15H8V13H11V10Z" />
    </svg>
  }
}
