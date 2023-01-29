#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_EmailBox)]
pub fn r#icon_email_box(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5,3A2,2 0 0,0 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V5A2,2 0 0,0 19,3H5M6.4,6.5H17.6C18.37,6.5 19,7.12 19,7.9V16.1A1.4,1.4 0 0,1 17.6,17.5H6.4C5.63,17.5 5,16.87 5,16.1V7.9C5,7.12 5.62,6.5 6.4,6.5M6,8V10L12,14L18,10V8L12,12L6,8Z" />
    </svg>
  }
}
