#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_VectorPointEdit)]
pub fn r#icon_vector_point_edit(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 9V15H15V9H9M11 11H13V13H11V11M21.2 13C21.1 13 20.9 13.1 20.8 13.2L19.8 14.2L21.9 16.3L22.9 15.3C23.1 15.1 23.1 14.7 22.9 14.5L21.6 13.2C21.4 13.1 21.3 13 21.2 13M19.1 14.8L13 20.9V23H15.1L21.2 16.8L19.1 14.8Z" />
    </svg>
  }
}
