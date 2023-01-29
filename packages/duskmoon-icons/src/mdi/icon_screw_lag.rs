#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ScrewLag)]
pub fn r#icon_screw_lag(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,19.3L14,16.6V20L12,22L10,20V19.3M14,12.7L10,15.4V17.4L9,18V19L15,15.1V14L14,14.7V12.7M7,2V5H17V2H7M9,6V9L10,9.7V13.4L9,14V15L15,11.1V10L14,10.7C14,10.7 14,10.3 14,9.7L15,9V6H9Z" />
    </svg>
  }
}
