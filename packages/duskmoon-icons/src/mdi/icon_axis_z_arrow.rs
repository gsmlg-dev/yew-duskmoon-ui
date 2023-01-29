#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AxisZArrow)]
pub fn r#icon_axis_z_arrow(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,2L16,6H13V13.82L22.39,19.25L21.39,21L12,15.56L2.61,21L1.61,19.25L11,13.82V6H8L12,2Z" />
    </svg>
  }
}
