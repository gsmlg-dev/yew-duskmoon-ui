#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SkiWater)]
pub fn r#icon_ski_water(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4.2 3.5C4.2 2.7 4.9 1.9 5.8 1.9C6.7 1.9 7.4 2.6 7.4 3.5S6.6 5 5.8 5 4.2 4.3 4.2 3.5M22 3.9L21.5 3L13.5 7.1L14 8L22 3.9M20.8 20.3L21.7 21.2C21.1 21.8 20.5 22.2 19.8 22.5S18.3 23 17.5 23H2V21.7H4.7L6.8 18.2L4.5 15L3.7 7.2C3.7 6.3 4.5 5.5 5.4 5.5C5.7 5.5 6 5.6 6.2 5.7L9.7 8.3L12 7.5L12.8 9.1L9.3 10.6C9.2 10.5 7.7 9.4 6.6 8.5L7 12L12.3 16.5L14 21.7H17.5C18.1 21.7 18.7 21.6 19.3 21.3C19.9 21.1 20.4 20.7 20.8 20.3M7 21.7H12L10.4 17.8L8.1 15.9L9.3 18.4L7 21.7Z" />
    </svg>
  }
}
