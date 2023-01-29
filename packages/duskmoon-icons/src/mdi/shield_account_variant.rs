#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ShieldAccountVariant)]
pub fn r#icon_shield_account_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17 11C17.3 11 17.7 11 18 11.1V6.3L10.5 3L3 6.3V11.2C3 15.7 6.2 20 10.5 21C11.1 20.9 11.6 20.7 12.1 20.5C11.4 19.5 11 18.3 11 17C11 13.7 13.7 11 17 11M17 13C14.8 13 13 14.8 13 17S14.8 21 17 21 21 19.2 21 17 19.2 13 17 13M17 14.4C17.6 14.4 18.1 14.9 18.1 15.5C18.1 16.1 17.6 16.6 17 16.6S15.9 16.1 15.9 15.5 16.4 14.4 17 14.4M17 19.8C16.1 19.8 15.3 19.3 14.8 18.6C14.9 17.9 16.3 17.5 17 17.5S19.2 17.9 19.2 18.6C18.7 19.3 17.9 19.8 17 19.8Z" />
    </svg>
  }
}