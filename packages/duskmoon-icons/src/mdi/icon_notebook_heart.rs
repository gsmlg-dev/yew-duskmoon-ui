#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_NotebookHeart)]
pub fn r#icon_notebook_heart(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 23.3L18.4 22.8C16.4 20.9 15 19.7 15 18.2C15 17 16 16 17.2 16C17.9 16 18.6 16.3 19 16.8C19.4 16.3 20.1 16 20.8 16C22 16 23 16.9 23 18.2C23 19.7 21.6 20.9 19.6 22.8L19 23.3M3 7V5H5V4C5 2.89 5.9 2 7 2H13V9L15.5 7.5L18 9V2H19C20.05 2 21 2.95 21 4V13.34C20.37 13.12 19.7 13 19 13C15.69 13 13 15.69 13 19C13 20.09 13.29 21.12 13.8 22H7C5.95 22 5 21.05 5 20V19H3V17H5V13H3V11H5V7H3M5 5V7H7V5H5M5 19H7V17H5V19M5 13H7V11H5V13Z" />
    </svg>
  }
}
