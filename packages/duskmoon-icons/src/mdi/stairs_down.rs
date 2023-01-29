#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_StairsDown)]
pub fn r#icon_stairs_down(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 6H22V9H18V13H14V17H10V21H3V18H7V14H11V10H15V6M4.83 8.34L10.34 2.83L12.17 4.66L6.66 10.17L8.5 12H3V6.5L4.83 8.34Z" />
    </svg>
  }
}
