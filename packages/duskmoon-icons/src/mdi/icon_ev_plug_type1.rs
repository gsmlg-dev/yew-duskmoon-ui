#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_EvPlugType1)]
pub fn r#icon_ev_plug_type1(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 9C13 10.1 13.9 11 15 11S17 10.1 17 9 16.1 7 15 7 13 7.9 13 9M7 9C7 10.1 7.9 11 9 11S11 10.1 11 9 10.1 7 9 7 7 7.9 7 9M9 13.2C9 12.6 8.4 12 7.8 12S6.6 12.6 6.6 13.2 7.2 14.4 7.8 14.4 9 13.9 9 13.2M14 16C14 14.9 13.1 14 12 14S10 14.9 10 16 10.9 18 12 18 14 17.1 14 16M12 4C16.4 4 20 7.6 20 12S16.4 20 12 20 4 16.4 4 12 7.6 4 12 4M12 2C6.5 2 2 6.5 2 12S6.5 22 12 22 22 17.5 22 12 17.5 2 12 2M17.5 13.2C17.5 12.5 16.9 12 16.3 12C15.6 12 15.1 12.6 15.1 13.2S15.7 14.4 16.3 14.4C16.9 14.5 17.5 13.9 17.5 13.2" />
    </svg>
  }
}