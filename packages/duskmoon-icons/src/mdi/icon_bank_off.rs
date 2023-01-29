#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BankOff)]
pub fn r#icon_bank_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.8 22.7L20.1 22H2V19H17.1L13 14.9V17H10V11.9L6.1 8H2V6L3.4 5.3L1.1 3L2.4 1.7L22.1 21.4L20.8 22.7M4 10V17H7V10H4M21 8V6L11.5 1L6.7 3.5L11.2 8H21M19 15.8V10H16V12.8L19 15.8Z" />
    </svg>
  }
}
