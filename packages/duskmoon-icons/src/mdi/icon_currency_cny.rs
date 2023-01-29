#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CurrencyCny)]
pub fn r#icon_currency_cny(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13.28 12H18V14H13V21H11V14H6V12H10.72L5 3H7.37L12 10.29L16.63 3H19L13.28 12Z" />
    </svg>
  }
}
