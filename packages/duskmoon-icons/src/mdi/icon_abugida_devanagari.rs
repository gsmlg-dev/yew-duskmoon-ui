#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_AbugidaDevanagari)]
pub fn r#icon_abugida_devanagari(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8 3V5H11C12.32 5 13.41 5.83 13.82 7H6V9H14V10H12C9.25 10 7 12.25 7 15C7 17.75 9.25 20 12 20C12.77 20 13.45 19.73 14 19.3V21H16V17H14C13.55 17.62 12.83 18 12 18C10.33 18 9 16.67 9 15C9 13.33 10.33 12 12 12H16V9H18V7H15.9C15.43 4.72 13.41 3 11 3H8Z" />
    </svg>
  }
}
