#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_StarBoxMultiple)]
pub fn r#icon_star_box_multiple(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4 6V20H18V22H3C2.4 22 2 21.6 2 21V6H4M8 2H20C21.11 2 22 2.9 22 4V16C22 17.11 21.11 18 20 18H8C6.9 18 6 17.11 6 16V4C6 2.9 6.9 2 8 2M16.3 13.3L15.9 10.7L17.8 8.8L15.2 8.4L14 6L12.8 8.4L10.2 8.8L12.1 10.6L11.6 13.2L14 12L16.3 13.3Z" />
    </svg>
  }
}
