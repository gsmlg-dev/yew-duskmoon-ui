#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_DivingHelmet)]
pub fn r#icon_diving_helmet(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16,12A4,4 0 0,1 12,16A4,4 0 0,1 8,12A4,4 0 0,1 12,8A4,4 0 0,1 16,12M19.45,7.89L18.07,8.58L18.32,9H20V15H18.33C17.84,16.03 17.11,16.92 16.21,17.6C17.85,18.11 19.1,18.91 19.67,19.86C18.69,21.1 15.62,22 12,22C8.38,22 5.31,21.1 4.33,19.86C4.9,18.91 6.15,18.11 7.79,17.6C6.89,16.92 6.16,16.03 5.67,15H4V9H5.68C6.37,7.54 7.54,6.37 9,5.68V4H15V5.68C15.68,6 16.29,6.46 16.82,7L18.55,6.14C19.93,5.42 20,4.1 20,2H22C22,4.06 22,6.62 19.45,7.89M17,12A5,5 0 0,0 12,7A5,5 0 0,0 7,12A5,5 0 0,0 12,17A5,5 0 0,0 17,12Z" />
    </svg>
  }
}
