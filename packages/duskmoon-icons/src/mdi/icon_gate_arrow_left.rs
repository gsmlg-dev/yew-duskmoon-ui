#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_GateArrowLeft)]
pub fn r#icon_gate_arrow_left(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 6H7V11H5V6H3V11H2V13H3V14.81C3.62 14.45 4.3 14.21 5 14.09V13H7V14.09C7.7 14.21 8.38 14.45 9 14.81V13H11V16.69C11.65 17.67 12 18.82 12 20C12 20.34 11.97 20.67 11.91 21H13V19H15V21H17V9H15V11H13V7H11V11H9V6M15 13V17H13V13H15M2 20L5 23V21H9V19H5V17L2 20Z" />
    </svg>
  }
}