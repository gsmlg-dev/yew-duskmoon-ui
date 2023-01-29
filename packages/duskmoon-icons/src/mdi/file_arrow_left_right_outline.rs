#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FileArrowLeftRightOutline)]
pub fn r#icon_file_arrow_left_right_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13.09 20C13.21 20.72 13.46 21.39 13.81 22H6C4.89 22 4 21.11 4 20V4C4 2.9 4.89 2 6 2H14L20 8V13.09C19.67 13.04 19.34 13 19 13C18.66 13 18.33 13.04 18 13.09V9H13V4H6V20H13.09M23 17L20 14.5V16H16V18H20V19.5L23 17M18 18.5L15 21L18 23.5V22H22V20H18V18.5Z" />
    </svg>
  }
}
