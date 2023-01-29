#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ImageMinusOutline)]
pub fn r#icon_image_minus_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15.4 14.21C14.47 14.91 13.75 15.88 13.35 17H6.5L9.25 13.47L11.21 15.83L13.96 12.29L15.4 14.21M5 19V5H19V13C19.7 13 20.37 13.13 21 13.35V5C21 3.9 20.11 3 19 3H5C3.9 3 3 3.9 3 5V19C3 20.11 3.9 21 5 21H13.35C13.13 20.37 13 19.7 13 19H5M15 18V20H23V18H15Z" />
    </svg>
  }
}
