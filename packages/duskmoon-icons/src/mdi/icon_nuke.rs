#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Nuke)]
pub fn r#icon_nuke(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.04,12H10V11H5.5A3.5,3.5 0 0,1 2,7.5A3.5,3.5 0 0,1 5.5,4C6.53,4 7.45,4.44 8.09,5.15C8.5,3.35 10.08,2 12,2C13.92,2 15.5,3.35 15.91,5.15C16.55,4.44 17.47,4 18.5,4A3.5,3.5 0 0,1 22,7.5A3.5,3.5 0 0,1 18.5,11H14.04V12M10,16.9V15.76H5V13.76H19V15.76H14.04V16.92L20,19.08C20.58,19.29 21,19.84 21,20.5A1.5,1.5 0 0,1 19.5,22H4.5A1.5,1.5 0 0,1 3,20.5C3,19.84 3.42,19.29 4,19.08L10,16.9Z" />
    </svg>
  }
}
