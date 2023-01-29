#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Lastpass)]
pub fn r#icon_lastpass(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14,12A2,2 0 0,1 16,10A2,2 0 0,1 18,12A2,2 0 0,1 16,14A2,2 0 0,1 14,12M8,12A2,2 0 0,1 10,10A2,2 0 0,1 12,12A2,2 0 0,1 10,14A2,2 0 0,1 8,12M2,12A2,2 0 0,1 4,10A2,2 0 0,1 6,12A2,2 0 0,1 4,14A2,2 0 0,1 2,12M22,5H20V19H22V5Z" />
    </svg>
  }
}
