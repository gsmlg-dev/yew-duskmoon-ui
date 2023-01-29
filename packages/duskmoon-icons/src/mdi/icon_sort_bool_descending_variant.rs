#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SortBoolDescendingVariant)]
pub fn r#icon_sort_bool_descending_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 7H22L18 3L14 7H17V21H19M5 13C3.89 13 3 13.89 3 15V19C3 20.11 3.89 21 5 21H9C10.11 21 11 20.11 11 19V15C11 13.89 10.11 13 9 13M9.2 14.5L10.26 15.55L6.27 19.5L3.74 16.95L4.81 15.9L6.28 17.39M5 3C3.89 3 3 3.89 3 5V9C3 10.11 3.89 11 5 11H9C10.11 11 11 10.11 11 9V5C11 3.89 10.11 3 9 3M5 5H9V9H5Z" />
    </svg>
  }
}
