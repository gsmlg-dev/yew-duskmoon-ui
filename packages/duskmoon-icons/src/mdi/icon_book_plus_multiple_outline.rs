#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BookPlusMultipleOutline)]
pub fn r#icon_book_plus_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13.09 20H5V6H3V20A2 2 0 0 0 5 22H13.81A5.5 5.5 0 0 1 13.09 20M19 2H9A2 2 0 0 0 7 4V16A2 2 0 0 0 9 18H13.09A5.5 5.5 0 0 1 13.81 16H9V4H11V10L13.5 7.75L16 10V4H19V13A6 6 0 0 1 21 13.34V4A2 2 0 0 0 19 2M20 15V18H23V20H20V23H18V20H15V18H18V15Z" />
    </svg>
  }
}
