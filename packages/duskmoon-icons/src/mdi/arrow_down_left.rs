#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ArrowDownLeft)]
pub fn r#icon_arrow_down_left(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 4V6H13.5C11 6 9 8 9 10.5V16.17L12.09 13.09L13.5 14.5L8 20L2.5 14.5L3.91 13.08L7 16.17V10.5C7 6.91 9.91 4 13.5 4H20Z" />
    </svg>
  }
}
