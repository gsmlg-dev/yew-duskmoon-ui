#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_KeyChange)]
pub fn r#icon_key_change(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7.5 2C9.5 2 11.1 3.2 11.7 5H21V8H18V11H15V8H11.7C11.1 9.8 9.4 11 7.5 11C5 11 3 9 3 6.5S5 2 7.5 2M7.5 5C6.7 5 6 5.7 6 6.5S6.7 8 7.5 8 9 7.3 9 6.5 8.3 5 7.5 5M7.5 13C9.5 13 11.1 14.2 11.7 16H21V19H20V22H18V19H16V22H13V19H11.7C11.1 20.8 9.4 22 7.5 22C5 22 3 20 3 17.5S5 13 7.5 13M7.5 16C6.7 16 6 16.7 6 17.5S6.7 19 7.5 19 9 18.3 9 17.5 8.3 16 7.5 16Z" />
    </svg>
  }
}
