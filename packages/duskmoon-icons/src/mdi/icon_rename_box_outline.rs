#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_RenameBoxOutline)]
pub fn r#icon_rename_box_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17 17H11.5L13.5 15H17M7 17V14.5L14.9 6.7C15.1 6.5 15.4 6.5 15.6 6.7L17.4 8.5C17.6 8.7 17.6 9 17.4 9.2L9.5 17M19 5V19H5V5H19M19 3H5C3.9 3 3 3.9 3 5V19C3 20.1 3.9 21 5 21H19C20.1 21 21 20.1 21 19V5C21 3.9 20.1 3 19 3Z" />
    </svg>
  }
}
