#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CurrentDc)]
pub fn r#icon_current_dc(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2,9V11H22V9H2M2,13V15H7V13H2M9,13V15H15V13H9M17,13V15H22V13H17Z" />
    </svg>
  }
}
