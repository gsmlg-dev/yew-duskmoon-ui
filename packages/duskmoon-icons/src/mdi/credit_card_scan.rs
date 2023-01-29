#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CreditCardScan)]
pub fn r#icon_credit_card_scan(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 6H6A2 2 0 0 0 4 8V16A2 2 0 0 0 6 18H18A2 2 0 0 0 20 16V8A2 2 0 0 0 18 6M18 12H6V9H18M2 4H6V2H2A2 2 0 0 0 0 4V8H2V4M22 2H18V4H22V8H24V4A2 2 0 0 0 22 2M2 16H0V20A2 2 0 0 0 2 22H6V20H2V16M22 20H18V22H22A2 2 0 0 0 24 20V16H22V20" />
    </svg>
  }
}
