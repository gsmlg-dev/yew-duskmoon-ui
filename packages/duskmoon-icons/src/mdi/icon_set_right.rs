#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SetRight)]
pub fn r#icon_set_right(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15,19C13.96,19 12.94,18.76 12,18.32C11.06,18.76 10.04,19 9,19A7,7 0 0,1 2,12A7,7 0 0,1 9,5C10.04,5 11.06,5.24 12,5.68C12.94,5.24 13.96,5 15,5A7,7 0 0,1 22,12A7,7 0 0,1 15,19M9,17L10,16.89C8.72,15.59 8,13.83 8,12C8,10.17 8.72,8.41 10,7.1L9,7A5,5 0 0,0 4,12A5,5 0 0,0 9,17M12,16C13.26,15.05 14,13.57 14,12C14,10.43 13.26,8.95 12,8C10.74,8.95 10,10.43 10,12C10,13.57 10.74,15.05 12,16Z" />
    </svg>
  }
}