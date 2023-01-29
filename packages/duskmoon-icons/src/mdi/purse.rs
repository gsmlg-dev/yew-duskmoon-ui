#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Purse)]
pub fn r#icon_purse(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22,19.1L21,10.5A1.65,1.65 0 0,0 19.3,9H16V5L14,3H10L8,5V9H4.7A1.65,1.65 0 0,0 3,10.5L2,19.1A1.74,1.74 0 0,0 3.7,21H20.3A1.74,1.74 0 0,0 22,19.1M10,5H14V9H10" />
    </svg>
  }
}
