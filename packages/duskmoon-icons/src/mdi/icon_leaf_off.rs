#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_LeafOff)]
pub fn r#icon_leaf_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.84 22.73L15.14 17.03C13.26 18.79 10.92 20 8 20C7.64 20 7.14 19.87 6.66 19.7L5.71 22L3.82 21.34C5.15 18.03 6.5 14.32 9.66 11.55L8.77 10.66C6.76 12.03 4.86 14.1 3.75 17.25C3.75 17.25 2 15.5 2 13.5C2 12 3.12 9.32 5.72 7.61L1.11 3L2.39 1.73C2.39 1.73 16.39 15.74 16.39 15.74L22.11 21.46L20.84 22.73M17 8C15.35 8.37 13.93 8.88 12.7 9.5L17.5 14.29C20.87 9.35 22 3 22 3C21.03 4.95 14.35 5.24 9.38 6.18L12.15 8.95C14.81 8 17 8 17 8Z" />
    </svg>
  }
}
