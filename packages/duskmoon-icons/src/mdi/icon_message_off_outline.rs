#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_MessageOffOutline)]
pub fn r#icon_message_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7.2 4L5.2 2H20C21.11 2 22 2.9 22 4V16C22 16.76 21.57 17.41 20.95 17.75L19.2 16H20V4H7.2M22.11 21.46L20.84 22.73L16.11 18H6L2 22V4C2 3.97 2 3.93 2 3.9L1.11 3L2.39 1.73L6.1 5.44L16.65 16H16.66L18.66 18H18.65L22.11 21.46M14.11 16L4 5.89V18L6 16H14.11Z" />
    </svg>
  }
}
