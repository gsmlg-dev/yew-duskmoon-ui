#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Polo)]
pub fn r#icon_polo(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 9.5C22 10.88 20.88 12 19.5 12S17 10.88 17 9.5 18.12 7 19.5 7 22 8.12 22 9.5M11 17V3H8V17H2L6 21H13V17H11M16 17H14V21H16V17Z" />
    </svg>
  }
}
