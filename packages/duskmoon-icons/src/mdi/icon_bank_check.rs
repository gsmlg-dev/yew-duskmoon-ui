#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BankCheck)]
pub fn r#icon_bank_check(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17.8 21.2L15 18.2L16.2 17L17.8 18.6L21.4 15L22.6 16.4L17.8 21.2M13 10H10V17H12.1C12.2 16.2 12.6 15.4 13 14.7V10M16 10V12.3C16.6 12.1 17.3 12 18 12C18.3 12 18.7 12 19 12.1V10H16M12.1 19H2V22H13.5C12.8 21.2 12.3 20.1 12.1 19M21 6L11.5 1L2 6V8H21V6M7 17V10H4V17H7Z" />
    </svg>
  }
}
