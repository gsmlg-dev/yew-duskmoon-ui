#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PagePrevious)]
pub fn r#icon_page_previous(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4,21H19A2,2 0 0,0 21,19V13H17V15L13,12L17,9V11H21V5A2,2 0 0,0 19,3H4A2,2 0 0,0 2,5V19A2,2 0 0,0 4,21M4,15H8V17H4V15M4,11H11V13H4V11M4,7H11V9H4V7M21,11H24V13H21V11Z" />
    </svg>
  }
}