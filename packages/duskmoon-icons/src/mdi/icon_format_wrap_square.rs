#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FormatWrapSquare)]
pub fn r#icon_format_wrap_square(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,7L17,17H7L12,7M3,3H21V5H3V3M3,7H6V9H3V7M21,7V9H18V7H21M3,11H6V13H3V11M21,11V13H18V11H21M3,15H6V17H3V15M21,15V17H18V15H21M3,19H21V21H3V19Z" />
    </svg>
  }
}
