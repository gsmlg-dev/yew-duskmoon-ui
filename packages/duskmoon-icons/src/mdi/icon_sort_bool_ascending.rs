#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SortBoolAscending)]
pub fn r#icon_sort_bool_ascending(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 17H22L18 21L14 17H17V3H19V17M7 3C4.79 3 3 4.79 3 7S4.79 11 7 11 11 9.21 11 7 9.21 3 7 3M7 9C5.9 9 5 8.1 5 7S5.9 5 7 5 9 5.9 9 7 8.1 9 7 9M7 13C4.79 13 3 14.79 3 17S4.79 21 7 21 11 19.21 11 17 9.21 13 7 13Z" />
    </svg>
  }
}
