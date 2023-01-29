#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_BasketballHoopOutline)]
pub fn r#icon_basketball_hoop_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21,2H3A2,2 0 0,0 1,4V16A2,2 0 0,0 3,18H6.57L8,23L10,21L12,23L14,21L16,23L17.43,18H21A2,2 0 0,0 23,16V4A2,2 0 0,0 21,2M21,16H18V14H17V9A2,2 0 0,0 15,7H9A2,2 0 0,0 7,9V14H6V16H3V4H21V16M9,14V9H15V14H9Z" />
    </svg>
  }
}
