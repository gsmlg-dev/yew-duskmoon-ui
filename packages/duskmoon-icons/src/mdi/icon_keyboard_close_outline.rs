#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_KeyboardCloseOutline)]
pub fn r#icon_keyboard_close_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 23L16 19H8M4 3C2.9 3 2 3.9 2 5V15C2 16.1 2.9 17 4 17H20C21.1 17 22 16.1 22 15V5C22 3.9 21.1 3 20 3H4M4 5H20V15H4V5M5 6V8H7V6H5M8 6V8H10V6H8M11 6V8H13V6H11M14 6V8H16V6H14M17 6V8H19V6H17M5 9V11H7V9H5M8 9V11H10V9H8M11 9V11H13V9H11M14 9V11H16V9H14M17 9V11H19V9H17M8 12V14H16V12H8Z" />
    </svg>
  }
}
