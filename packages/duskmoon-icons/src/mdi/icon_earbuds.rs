#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Earbuds)]
pub fn r#icon_earbuds(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10 8V20C10 20.55 9.55 21 9 21H8C7.45 21 7 20.55 7 20V13.27C6.32 13.72 5.61 14 5 14C3 14 2 12 2 11V6C2 5 3 3 5 3S10 6 10 8M16.5 3C13.46 3 11 5.46 11 8.5S13.46 14 16.5 14 22 11.54 22 8.5 19.54 3 16.5 3M14.5 14.68V20C14.5 20.55 14.95 21 15.5 21H17.5C18.05 21 18.5 20.55 18.5 20V14.68C17.87 14.89 17.2 15 16.5 15S15.13 14.89 14.5 14.68Z" />
    </svg>
  }
}
