#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_NetworkStrength4Cog)]
pub fn r#icon_network_strength_4_cog(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17 13C16.9 13 16.8 13.1 16.7 13.2L16.5 14.5C16.2 14.6 15.9 14.8 15.7 15L14.5 14.5C14.4 14.5 14.3 14.5 14.2 14.6L13.2 16.3C13.1 16.4 13.2 16.5 13.3 16.6L14.4 17.4V18.4L13.3 19.2C13.2 19.3 13.2 19.4 13.2 19.5L14.2 21.2C14.3 21.3 14.4 21.3 14.5 21.3L15.7 20.8C16 21 16.2 21.2 16.5 21.3L16.7 22.6C16.7 22.7 16.8 22.8 17 22.8H19C19.1 22.8 19.2 22.7 19.2 22.6L19.4 21.3C19.7 21.2 20 21 20.2 20.8L21.4 21.3C21.5 21.3 21.7 21.3 21.7 21.2L22.7 19.5C22.8 19.4 22.7 19.3 22.6 19.2L21.5 18.4V17.9 17.4L22.6 16.6C22.7 16.5 22.7 16.4 22.7 16.3L21.7 14.6C21.6 14.5 21.5 14.5 21.4 14.5L20.3 15C20 14.8 19.8 14.6 19.4 14.5L19.2 13.2C19.2 13.1 19.1 13 19 13H17M18 16.5C18.8 16.5 19.5 17.2 19.5 18S18.8 19.5 18 19.5C17.2 19.5 16.5 18.8 16.5 18S17.2 16.5 18 16.5M11.7 21H1L21 1V11.7C20.1 11.3 19.1 11 18 11C14.1 11 11 14.1 11 18C11 19.1 11.2 20.1 11.7 21Z" />
    </svg>
  }
}
