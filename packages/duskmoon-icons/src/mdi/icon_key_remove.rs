#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_KeyRemove)]
pub fn r#icon_key_remove(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7.5 3C9.5 3 11.1 4.2 11.7 6H21V9H18V12H15V9H11.7C11.1 10.8 9.4 12 7.5 12C5 12 3 10 3 7.5S5 3 7.5 3M7.5 6C6.7 6 6 6.7 6 7.5S6.7 9 7.5 9 9 8.3 9 7.5 8.3 6 7.5 6M14.6 14L16 15.4L13.4 18L16 20.6L14.6 22L12 19.4L9.4 22L8 20.6L10.6 18L8 15.4L9.4 14L12 16.6L14.6 14Z" />
    </svg>
  }
}