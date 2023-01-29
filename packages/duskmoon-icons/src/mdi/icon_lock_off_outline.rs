#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_LockOffOutline)]
pub fn r#icon_lock_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 5.82L7.36 4.16C8.09 2.31 9.89 1 12 1C14.76 1 17 3.24 17 6V8H18C19.11 8 20 8.9 20 10V16.8L18 14.8V10H13.2L11.2 8H15V6C15 4.34 13.66 3 12 3C10.41 3 9.11 4.25 9 5.82M22.11 21.46L20.84 22.73L19.46 21.35C19.1 21.75 18.58 22 18 22H6C4.89 22 4 21.1 4 20V10C4 8.89 4.89 8 6 8H6.11L1.11 3L2.39 1.73L22.11 21.46M18 19.89L13.85 15.74C13.56 16.5 12.84 17 12 17C10.89 17 10 16.1 10 15C10 14.15 10.5 13.44 11.26 13.15L8.11 10H6V20H18V19.89Z" />
    </svg>
  }
}
