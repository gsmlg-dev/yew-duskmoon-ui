#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_TruckAlert)]
pub fn r#icon_truck_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 4C1.9 4 1 4.9 1 6V17H3C3 18.7 4.3 20 6 20S9 18.7 9 17H15C15 18.7 16.3 20 18 20S21 18.7 21 17H23V12L20 8H17V4H3M8 6H10V10H8V6M17 9.5H19.5L21.5 12H17V9.5M8 12H10V14H8V12M6 15.5C6.8 15.5 7.5 16.2 7.5 17S6.8 18.5 6 18.5 4.5 17.8 4.5 17 5.2 15.5 6 15.5M18 15.5C18.8 15.5 19.5 16.2 19.5 17S18.8 18.5 18 18.5 16.5 17.8 16.5 17 17.2 15.5 18 15.5Z" />
    </svg>
  }
}
