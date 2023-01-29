#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TableFurniture)]
pub fn r#icon_table_furniture(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2 7H22V10H20L21 19H18.5L17.94 14H6.06L5.5 19H3L4 10H2V7M17.5 10H6.5L6.29 12H17.71L17.5 10Z" />
    </svg>
  }
}
