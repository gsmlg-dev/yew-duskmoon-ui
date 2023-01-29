#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FolderMultiplePlusOutline)]
pub fn r#icon_folder_multiple_plus_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 4C23.1 4 24 4.9 24 6V16C24 17.1 23.1 18 22 18H6C4.9 18 4 17.1 4 16V4C4 2.9 4.9 2 6 2H12L14 4H22M2 6V20H20V22H2C.9 22 0 21.1 0 20V6H2M6 6V16H22V6H6M14 10H16V8H18V10H20V12H18V14H16V12H14V10Z" />
    </svg>
  }
}
