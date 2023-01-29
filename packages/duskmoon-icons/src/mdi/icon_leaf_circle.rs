#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_LeafCircle)]
pub fn r#icon_leaf_circle(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 2C6.5 2 2 6.5 2 12S6.5 22 12 22 22 17.5 22 12 17.5 2 12 2M9.6 17.2C9.38 17.2 9.08 17.12 8.8 17L8.23 18.4L7.09 18L7.25 17.61C8.45 14.59 9.83 11.15 15 10C15 10 9 10 7.05 15.55C7.05 15.55 6 14.5 6 13.3S7.2 9.55 10.2 8.95C11.05 8.78 12 8.65 12.94 8.5C15.3 8.18 17.57 7.86 18 7C18 7 16.2 17.2 9.6 17.2Z" />
    </svg>
  }
}