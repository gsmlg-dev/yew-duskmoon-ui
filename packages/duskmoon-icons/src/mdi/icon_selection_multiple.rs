#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SelectionMultiple)]
pub fn r#icon_selection_multiple(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 6V4H15V6H11M6 9H8V13H6V9M6 6C6 4.89 6.9 4 8 4H9V6H8V7H6V6M8 18C6.89 18 6 17.1 6 16V15H8V16H9V18H8M18 4C19.11 4 20 4.9 20 6V7H18V6H17V4H18M15 16V18H11V16H15M20 13H18V9H20V13M20 16C20 17.11 19.1 18 18 18H17V16H18V15H20V16M3 8H4V20H16V21C16 21.54 15.57 22 15.03 22H15L3 22C2.45 22 2 21.55 2 21V9C2 8.45 2.45 8 3 8Z" />
    </svg>
  }
}
