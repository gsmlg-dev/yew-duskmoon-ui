#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_WeatherTornado)]
pub fn r#icon_weather_tornado(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21,5H3A1,1 0 0,1 2,4A1,1 0 0,1 3,3H21A1,1 0 0,1 22,4A1,1 0 0,1 21,5M20,8A1,1 0 0,0 19,7H5A1,1 0 0,0 4,8A1,1 0 0,0 5,9H19A1,1 0 0,0 20,8M21,12A1,1 0 0,0 20,11H10A1,1 0 0,0 9,12A1,1 0 0,0 10,13H20A1,1 0 0,0 21,12M16,16A1,1 0 0,0 15,15H9A1,1 0 0,0 8,16A1,1 0 0,0 9,17H15A1,1 0 0,0 16,16M13,20A1,1 0 0,0 12,19H10A1,1 0 0,0 9,20A1,1 0 0,0 10,21H12A1,1 0 0,0 13,20Z" />
    </svg>
  }
}
