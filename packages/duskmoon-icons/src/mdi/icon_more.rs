#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_More)]
pub fn r#icon_more(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19,13.5A1.5,1.5 0 0,1 17.5,12A1.5,1.5 0 0,1 19,10.5A1.5,1.5 0 0,1 20.5,12A1.5,1.5 0 0,1 19,13.5M14,13.5A1.5,1.5 0 0,1 12.5,12A1.5,1.5 0 0,1 14,10.5A1.5,1.5 0 0,1 15.5,12A1.5,1.5 0 0,1 14,13.5M9,13.5A1.5,1.5 0 0,1 7.5,12A1.5,1.5 0 0,1 9,10.5A1.5,1.5 0 0,1 10.5,12A1.5,1.5 0 0,1 9,13.5M22,3H7C6.31,3 5.77,3.35 5.41,3.88L0,12L5.41,20.11C5.77,20.64 6.37,21 7.06,21H22A2,2 0 0,0 24,19V5C24,3.89 23.1,3 22,3Z" />
    </svg>
  }
}
