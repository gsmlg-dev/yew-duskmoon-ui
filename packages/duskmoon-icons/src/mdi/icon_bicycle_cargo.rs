#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BicycleCargo)]
pub fn r#icon_bicycle_cargo(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 11.5V10L13.5 9V5H9V6.5H12V14.3H11L9 10C9.3 9.9 9.5 9.6 9.5 9.3C9.5 8.9 9.2 8.5 8.8 8.5H6.8C6.3 8.5 6 8.8 6 9.2S6.3 10 6.8 10H7.4L9.4 14.2H7.9C7.6 12.4 6 11 4 11C1.8 11 0 12.8 0 15S1.8 19 4 19C6 19 7.6 17.6 7.9 15.8H16.5C16.7 13.4 18.6 11.5 21 11.5M6.4 15.8C6.1 16.8 5.1 17.6 4 17.6C2.6 17.6 1.5 16.5 1.5 15.1S2.6 12.6 4 12.6C5.1 12.6 6.1 13.3 6.4 14.4H4V15.9H6.4M21 13C19.3 13 18 14.3 18 16S19.3 19 21 19 24 17.7 24 16 22.7 13 21 13M21 17.5C20.2 17.5 19.5 16.8 19.5 16S20.2 14.5 21 14.5 22.5 15.2 22.5 16 21.8 17.5 21 17.5Z" />
    </svg>
  }
}
