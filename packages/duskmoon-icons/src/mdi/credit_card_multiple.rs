#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CreditCardMultiple)]
pub fn r#icon_credit_card_multiple(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 9V6H7V9H21M21 3A2 2 0 0 1 23 5V15A2 2 0 0 1 21 17H7A2 2 0 0 1 5 15V5A2 2 0 0 1 7 3H21M3 19H18V21H3A2 2 0 0 1 1 19V8H3Z" />
    </svg>
  }
}
