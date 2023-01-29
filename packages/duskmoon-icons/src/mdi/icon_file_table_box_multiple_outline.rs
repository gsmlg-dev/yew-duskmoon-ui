#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FileTableBoxMultipleOutline)]
pub fn r#icon_file_table_box_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 5V21H19V23H3C1.9 23 1 22.1 1 21V5H3M21 1H7C5.89 1 5 1.89 5 3V17C5 18.1 5.9 19 7 19H21C22.11 19 23 18.11 23 17V3C23 1.9 22.1 1 21 1M21 17H7V3H21V17M11 14H8V16H11V14M15 14H12V16H15V14M11 11H8V13H11V11M15 11H12V13H15V11M11 8H8V10H11V8M15 8H12V10H15V8Z" />
    </svg>
  }
}
