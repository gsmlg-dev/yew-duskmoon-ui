#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ArrowUUpRight)]
pub fn r#icon_arrow_u_up_right(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.5 9.5L20.09 10.92L17 7.83V13.5C17 17.09 14.09 20 10.5 20S4 17.09 4 13.5V6H6V13.5C6 16 8 18 10.5 18S15 16 15 13.5V7.83L11.91 10.91L10.5 9.5L16 4L21.5 9.5Z" />
    </svg>
  }
}
