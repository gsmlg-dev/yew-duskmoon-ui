#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AxisZRotateCounterclockwise)]
pub fn r#icon_axis_z_rotate_counterclockwise(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,12L14,16L10,20V16.9C5.44,16.44 2,14.42 2,12C2,9.58 5.44,7.56 10,7.1V9.09C6.55,9.43 4,10.6 4,12C4,13.4 6.55,14.57 10,14.91V12M20,12C20,10.6 17.45,9.43 14,9.09V7.1C18.56,7.56 22,9.58 22,12C22,14.16 19.26,16 15.42,16.7L16.12,16L14.92,14.79C17.89,14.36 20,13.27 20,12M11,2H13V13L11,11V2M11,22V21L13,19V22H11Z" />
    </svg>
  }
}
