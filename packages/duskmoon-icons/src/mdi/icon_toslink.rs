#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Toslink)]
pub fn r#icon_toslink(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 10V7L17 4H7L4 7V10C2.9 10 2 10.9 2 12S2.9 14 4 14V18C4 19.1 4.9 20 6 20H18C19.1 20 20 19.1 20 18V14C21.11 14 22 13.11 22 12S21.11 10 20 10M12 16C9.79 16 8 14.21 8 12S9.79 8 12 8 16 9.79 16 12 14.21 16 12 16M14 12C14 13.11 13.11 14 12 14S10 13.11 10 12 10.9 10 12 10 14 10.9 14 12Z" />
    </svg>
  }
}
