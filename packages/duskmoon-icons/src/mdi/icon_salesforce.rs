#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Salesforce)]
pub fn r#icon_salesforce(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.38,6.37C17.68,6.37 17.07,6.58 16.47,6.78C15.77,5.57 14.46,4.77 13.05,4.77C11.95,4.77 10.95,5.27 10.24,5.97C9.44,4.97 8.23,4.26 6.83,4.26C4.5,4.26 2.5,6.17 2.5,8.5C2.5,9.09 2.71,9.69 2.91,10.29C1.8,10.9 1,12.1 1,13.5C1,15.5 2.61,17.22 4.62,17.22C4.92,17.22 5.22,17.22 5.42,17.12C5.82,18.63 7.33,19.74 9.14,19.74C10.84,19.74 12.25,18.73 12.75,17.32C13.26,17.53 13.76,17.73 14.26,17.73C15.57,17.73 16.77,17 17.37,15.92C17.68,16 18,16 18.28,16C20.89,16 23,13.91 23,11.2C23.1,8.5 21,6.37 18.38,6.37Z" />
    </svg>
  }
}
