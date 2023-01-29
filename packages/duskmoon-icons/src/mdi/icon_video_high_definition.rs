#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_VideoHighDefinition)]
pub fn r#icon_video_high_definition(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 10V14C14 14.3 13.8 14.5 13.5 14.5H12.5V9.5H13.5C13.8 9.5 14 9.7 14 10M17 10.5V7C17 6.4 16.6 6 16 6H4C3.4 6 3 6.4 3 7V17C3 17.6 3.4 18 4 18H16C16.6 18 17 17.6 17 17V13.5L21 17.5V6.5L17 10.5M9.5 16H8V12.8H6V16H4.5V8H6V11.2H8V8H9.5V16M15.5 14.5C15.5 15.3 14.8 16 14 16H11V8H14C14.8 8 15.5 8.7 15.5 9.5V14.5Z" />
    </svg>
  }
}
