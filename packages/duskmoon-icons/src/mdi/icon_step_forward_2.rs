#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_StepForward2)]
pub fn r#icon_step_forward_2(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7,5H10V19H7V5M12,5L23,12L12,19V5M2,5H5V19H2V5Z" />
    </svg>
  }
}
