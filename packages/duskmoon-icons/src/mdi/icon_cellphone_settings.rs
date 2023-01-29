#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CellphoneSettings)]
pub fn r#icon_cellphone_settings(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16,16H8V4H16M16,0H8A2,2 0 0,0 6,2V18A2,2 0 0,0 8,20H16A2,2 0 0,0 18,18V2A2,2 0 0,0 16,0M15,24H17V22H15M11,24H13V22H11M7,24H9V22H7V24Z" />
    </svg>
  }
}
