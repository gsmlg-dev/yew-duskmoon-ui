#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FlagVariantOffOutline)]
pub fn r#icon_flag_variant_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.1 21.5L2.4 1.7L1.1 3L5 6.9V21H7V14C7 14 7.8 12.4 10.2 12.1L20.9 22.8L22.1 21.5M7 11.5V8.9L8.7 10.6C7.7 11 7 11.5 7 11.5M9.4 6.2L7.8 4.6C8.7 4.3 9.8 4 11 4C14 4 14 6 16 6C19 6 20 4 20 4V12C20 12 19.2 13.5 17.1 13.9L15 11.8C15.3 11.9 15.6 12 16 12C18 12 18 11 18 11V7.5C18 7.5 17 8 16 8C14 8 13 6 11 6C10.5 6 9.9 6.1 9.4 6.2Z" />
    </svg>
  }
}