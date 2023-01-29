#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Peanut)]
pub fn r#icon_peanut(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16 12.77A2 2 0 0 1 15.86 10.12A5 5 0 0 0 13.04 2.12A5.74 5.74 0 0 0 12 2A5 5 0 0 0 7 7A5 5 0 0 0 8.11 10.12A2 2 0 0 1 8 12.72A5.5 5.5 0 0 0 10.81 21.87A5.42 5.42 0 0 0 12 22A5.5 5.5 0 0 0 16 12.77M13 5A1 1 0 1 1 12 6A1 1 0 0 1 13 5M11 18A1 1 0 1 1 12 17A1 1 0 0 1 11 18M12 15A1 1 0 1 1 13 16A1 1 0 0 1 12 15M14 19A1 1 0 1 1 15 18A1 1 0 0 1 14 19Z" />
    </svg>
  }
}
