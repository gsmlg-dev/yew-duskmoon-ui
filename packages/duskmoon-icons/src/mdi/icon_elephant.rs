#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Elephant)]
pub fn r#icon_elephant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.5,15.5A0.5,0.5 0 0,1 19,16A0.5,0.5 0 0,1 18.5,15.5V8.5C18.5,6.57 16.43,5 14.5,5H6A4,4 0 0,0 2,9V19H6V15H11V19H15V14.5A0.5,0.5 0 0,1 15.5,14A0.5,0.5 0 0,1 16,14.5V16A3,3 0 0,0 19,19A3,3 0 0,0 22,16V14H19.5V15.5Z" />
    </svg>
  }
}
