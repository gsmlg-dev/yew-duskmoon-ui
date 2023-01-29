#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ViewSequentialOutline)]
pub fn r#icon_view_sequential_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 5V19H21V5H3M19 7V9H5V7H19M19 11V13H5V11H19M5 17V15H19V17H5Z" />
    </svg>
  }
}
