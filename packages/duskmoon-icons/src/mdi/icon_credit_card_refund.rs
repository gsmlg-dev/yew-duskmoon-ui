#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CreditCardRefund)]
pub fn r#icon_credit_card_refund(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 11H6A2 2 0 0 0 4 13V21A2 2 0 0 0 6 23H18A2 2 0 0 0 20 21V13A2 2 0 0 0 18 11M18 17H6V14H18M17 5V10H15.5V6.5H9.88L12.3 8.93L11.24 10L7 5.75L11.24 1.5L12.3 2.57L9.88 5Z" />
    </svg>
  }
}
