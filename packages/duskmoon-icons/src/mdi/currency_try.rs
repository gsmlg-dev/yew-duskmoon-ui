#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CurrencyTry)]
pub fn r#icon_currency_try(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 8.76V3H11V7.5L15 5V7.36L11 9.87L11 12.22L15 9.72V12.08L11 14.59V19C13.76 19 16 16.76 16 14H18C18 17.87 14.87 21 11 21H9V15.84L6 17.72V15.36L9 13.5V11.12L6 13V10.64L9 8.76Z" />
    </svg>
  }
}