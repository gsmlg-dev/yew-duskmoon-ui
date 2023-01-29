#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PowerSocketCh)]
pub fn r#icon_power_socket_ch(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.78 2H4.22C3 2 2 3 2 4.22V19.78C2 21 3 22 4.22 22H19.78C21 22 22 21 22 19.78V4.22C22 3 21 2 19.78 2M20 13L16.55 16.45C16.19 16.81 15.75 17 15.3 17H8.7C8.25 17 7.81 16.81 7.45 16.45L4 13C3.5 12.5 3.5 11.5 4 11L7.45 7.55C7.81 7.19 8.25 7 8.7 7H15.3C15.75 7 16.19 7.19 16.55 7.55L20 11C20.5 11.5 20.5 12.5 20 13M10.5 14C10.5 13.17 11.17 12.5 12 12.5S13.5 13.17 13.5 14 12.83 15.5 12 15.5 10.5 14.83 10.5 14M15.5 12C15.5 11.17 16.17 10.5 17 10.5S18.5 11.17 18.5 12 17.83 13.5 17 13.5 15.5 12.83 15.5 12M5.5 12C5.5 11.17 6.17 10.5 7 10.5S8.5 11.17 8.5 12 7.83 13.5 7 13.5 5.5 12.83 5.5 12Z" />
    </svg>
  }
}
