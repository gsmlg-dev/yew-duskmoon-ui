#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AlphabetAurebesh)]
pub fn r#icon_alphabet_aurebesh(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 4V11H14.23L22 4H19L13.46 9H5V4H3M3 13V20H5V15H13.46L19 20H22L14.23 13H3Z" />
    </svg>
  }
}
