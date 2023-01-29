#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ViewParallelOutline)]
pub fn r#icon_view_parallel_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 3H5V21H19V3M17 19H15V5H17V19M13 19H11V5H13V19M7 5H9V19H7V5Z" />
    </svg>
  }
}
