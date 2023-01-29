#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FridgeOff)]
pub fn r#icon_fridge_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.11 21.46L2.39 1.73L1.11 3L5 6.89V9H7.11L8.11 10H5V19C5 20.11 5.9 21 7 21V22H9V21H15V22H17V21C17.58 21 18.1 20.75 18.46 20.35L20.84 22.73L22.11 21.46M10 15H8V12H10V15M19 15.8L13.2 10H19V15.8M8.2 5L5.7 2.5C6.05 2.19 6.5 2 7 2H17C18.11 2 19 2.9 19 4V9H12.2L10 6.8V5H8.2Z" />
    </svg>
  }
}
