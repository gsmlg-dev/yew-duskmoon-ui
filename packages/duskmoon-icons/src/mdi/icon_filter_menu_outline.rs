#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FilterMenuOutline)]
pub fn r#icon_filter_menu_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 18.88A1 1 0 0 1 11.71 19.71A1 1 0 0 1 10.3 19.71L6.3 15.71A1 1 0 0 1 6 14.87V9.75L1.21 3.62A1 1 0 0 1 1.38 2.22A1 1 0 0 1 2 2H16A1 1 0 0 1 16.62 2.22A1 1 0 0 1 16.79 3.62L12 9.75V18.88M4 4L8 9.06V14.58L10 16.58V9.05L14 4M13 16L18 21L23 16Z" />
    </svg>
  }
}
