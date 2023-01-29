#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ArrowUpRightBold)]
pub fn r#icon_arrow_up_right_bold(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 21V17H10.5C12.43 17 14 15.43 14 13.5V11H10L16 4L22 11H18V13.5C18 17.64 14.64 21 10.5 21H3Z" />
    </svg>
  }
}
