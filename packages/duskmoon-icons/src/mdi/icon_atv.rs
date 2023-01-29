#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Atv)]
pub fn r#icon_atv(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 11C19.8 11 19.6 11 19.5 11.1L17.4 9H20V6L16.3 7.9L13.4 5H9V7H12.6L14.6 9H11L7 11L5 9H0V11H4C1.8 11 0 12.8 0 15S1.8 19 4 19 8 17.2 8 15L10 17H13L16.5 10.9L17.5 11.9C16.6 12.6 16 13.8 16 15C16 17.2 17.8 19 20 19S24 17.2 24 15 22.2 11 20 11M4 17C2.9 17 2 16.1 2 15S2.9 13 4 13 6 13.9 6 15 5.1 17 4 17M20 17C18.9 17 18 16.1 18 15S18.9 13 20 13 22 13.9 22 15 21.1 17 20 17Z" />
    </svg>
  }
}
