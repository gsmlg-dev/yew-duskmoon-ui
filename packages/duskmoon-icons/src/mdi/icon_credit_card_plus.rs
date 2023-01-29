#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CreditCardPlus)]
pub fn r#icon_credit_card_plus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 4H4A2 2 0 0 0 2 6V18A2 2 0 0 0 4 20H13.09A5.47 5.47 0 0 1 13 19A6 6 0 0 1 19 13A5.88 5.88 0 0 1 22 13.81V6A2 2 0 0 0 20 4M20 11H4V8H20M20 15V18H23V20H20V23H18V20H15V18H18V15Z" />
    </svg>
  }
}
