#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Ninja)]
pub fn r#icon_ninja(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7.75,13C7.74,12.65 7.9,12.31 8.17,12.08C8.92,12.24 9.62,12.55 10.25,13C10.25,13.68 9.69,14.24 9,14.24C8.31,14.24 7.76,13.69 7.75,13M13.75,13C14.38,12.56 15.08,12.25 15.83,12.09C16.1,12.32 16.26,12.66 16.25,13C16.25,13.7 15.69,14.26 15,14.26C14.31,14.26 13.75,13.7 13.75,13V13M12,9C9.23,8.96 6.5,9.65 4.07,11L4,12C4,13.23 4.29,14.44 4.84,15.54C7.21,15.18 9.6,15 12,15C14.4,15 16.79,15.18 19.16,15.54C19.71,14.44 20,13.23 20,12L19.93,11C17.5,9.65 14.77,8.96 12,9M12,2A10,10 0 0,1 22,12A10,10 0 0,1 12,22A10,10 0 0,1 2,12A10,10 0 0,1 12,2Z" />
    </svg>
  }
}