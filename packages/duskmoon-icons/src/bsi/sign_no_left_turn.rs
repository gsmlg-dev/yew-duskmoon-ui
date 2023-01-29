#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(BS_SignNoLeftTurn)]
pub fn r#icon_sign_no_left_turn(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M0 8a8 8 0 1 0 16 0A8 8 0 0 0 0 8Zm3.416 5.29 5.988-5.987c.362.274.596.708.596 1.197V11h1V8.5c0-.765-.344-1.45-.885-1.908l3.176-3.176a7 7 0 0 1-9.874 9.874Zm-.707-.706a7 7 0 0 1 9.874-9.874L9.196 6.097A2.501 2.501 0 0 0 8.5 6H7V4.534a.25.25 0 0 0-.41-.192L4.23 6.308a.25.25 0 0 0 0 .384l2.36 1.966a.265.265 0 0 0 .026.02l-3.907 3.906ZM8.293 7 7 8.293V7h1.293Z"/>
    </svg>
  }
}