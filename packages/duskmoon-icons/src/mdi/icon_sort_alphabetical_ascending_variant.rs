#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SortAlphabeticalAscendingVariant)]
pub fn r#icon_sort_alphabetical_ascending_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9.25 5L12.5 1.75L15.75 5H9.25M8.89 14.3H6L5.28 17H2.91L6 7H9L12.13 17H9.67L8.89 14.3M6.33 12.68H8.56L7.93 10.56L7.67 9.59L7.42 8.63H7.39L7.17 9.6L6.93 10.58L6.33 12.68M13.05 17V15.74L17.8 8.97V8.91H13.5V7H20.73V8.34L16.09 15V15.08H20.8V17H13.05Z" />
    </svg>
  }
}