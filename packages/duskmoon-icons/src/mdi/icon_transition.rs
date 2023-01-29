#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Transition)]
pub fn r#icon_transition(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15,2A7,7 0 0,1 22,9C22,11.71 20.46,14.05 18.22,15.22C17.55,16.5 16.5,17.55 15.22,18.22C14.05,20.46 11.71,22 9,22A7,7 0 0,1 2,15C2,12.29 3.54,9.95 5.78,8.78C6.45,7.5 7.5,6.45 8.78,5.78C9.95,3.54 12.29,2 15,2M12,19A7,7 0 0,1 5,12C4.37,12.84 4,13.87 4,15A5,5 0 0,0 9,20C10.13,20 11.16,19.63 12,19M15,16A7,7 0 0,1 8,9H8C7.37,9.84 7,10.87 7,12A5,5 0 0,0 12,17C13.13,17 14.16,16.63 15,16V16M15,4C13.87,4 12.84,4.37 12,5V5A7,7 0 0,1 19,12H19C19.63,11.16 20,10.13 20,9A5,5 0 0,0 15,4M10,9A5,5 0 0,0 15,14C15.6,14 16.17,13.9 16.7,13.7C16.9,13.17 17,12.6 17,12A5,5 0 0,0 12,7C11.4,7 10.83,7.1 10.3,7.3C10.1,7.83 10,8.4 10,9Z" />
    </svg>
  }
}
