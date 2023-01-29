#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_HomeMinus)]
pub fn r#icon_home_minus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 17V19H14V17H22M12 3L22 12H18C14.69 12 12 14.69 12 18C12 18.7 12.12 19.37 12.34 20H5V12H2L12 3Z" />
    </svg>
  }
}
