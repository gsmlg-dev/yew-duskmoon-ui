#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Rewind45)]
pub fn r#icon_rewind_45(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.5 3C17.1 3 21.1 6 22.5 10.2L20.1 11C19 7.8 16 5.5 12.5 5.5C10.5 5.5 8.8 6.2 7.4 7.4L10 10H3V3L5.6 5.6C7.4 4 9.9 3 12.5 3M13 12H19V14H15V16H17C18.1 16 19 16.9 19 18V20C19 21.1 18.1 22 17 22H13V20H17V18H13V12M5 12V18H9V22H11V12H9V16H7V12H5Z" />
    </svg>
  }
}
