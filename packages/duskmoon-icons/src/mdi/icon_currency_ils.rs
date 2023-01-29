#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CurrencyIls)]
pub fn r#icon_currency_ils(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22,16A5,5 0 0,1 17,21H8V9H10V19H17A3,3 0 0,0 20,16V3H22V16M16,8V15H14V8A3,3 0 0,0 11,5H4V21H2V3H11A5,5 0 0,1 16,8Z" />
    </svg>
  }
}
