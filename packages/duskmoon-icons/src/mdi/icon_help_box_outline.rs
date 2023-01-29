#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_HelpBoxOutline)]
pub fn r#icon_help_box_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 18H13V16H11V18M12 6C9.8 6 8 7.8 8 10H10C10 8.9 10.9 8 12 8S14 8.9 14 10C14 12 11 11.8 11 15H13C13 12.8 16 12.5 16 10C16 7.8 14.2 6 12 6M19 5V19H5V5H19M19 3H5C3.9 3 3 3.9 3 5V19C3 20.1 3.9 21 5 21H19C20.1 21 21 20.1 21 19V5C21 3.9 20.1 3 19 3Z" />
    </svg>
  }
}
