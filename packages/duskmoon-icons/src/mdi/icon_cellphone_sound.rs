#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CellphoneSound)]
pub fn r#icon_cellphone_sound(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.1,8.7C20.9,10.5 20.9,13.3 19.1,15.2L20.1,16.2C22.6,13.9 22.6,10.1 20.1,7.7L19.1,8.7M18,9.8L17,10.8C17.5,11.5 17.5,12.4 17,13.1L18,14.1C19.2,12.9 19.2,11.1 18,9.8M14,1H4A2,2 0 0,0 2,3V21A2,2 0 0,0 4,23H14A2,2 0 0,0 16,21V3A2,2 0 0,0 14,1M14,20H4V4H14V20Z" />
    </svg>
  }
}
