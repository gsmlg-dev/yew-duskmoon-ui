#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(BS_SignNoLeftTurnFill)]
pub fn r#icon_sign_no_left_turn_fill(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2 13.292A8 8 0 0 1 13.293 2L9.195 6.099A2.501 2.501 0 0 0 8.5 6H7V4.534a.25.25 0 0 0-.41-.192L4.23 6.308a.25.25 0 0 0 0 .384l2.36 1.966a.265.265 0 0 0 .026.02L2 13.291Zm.708.708A8 8 0 0 0 14 2.707l-3.885 3.884C10.656 7.05 11 7.735 11 8.5V11h-1V8.5c0-.489-.234-.923-.596-1.197l-6.696 6.696Z"/>
  <path d="M8.293 7 7 8.293V7h1.293Z"/>
    </svg>
  }
}
