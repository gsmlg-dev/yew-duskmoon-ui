#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BorderRightVariant)]
pub fn r#icon_border_right_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11,5H13V3H11M15,5H17V3H15M15,21H17V19H15M19,21H21V3H19M3,9H5V7H3M3,17H5V15H3M3,13H5V11H3M11,21H13V19H11M3,21H5V19H3M7,5H9V3H7M3,5H5V3H3M7,21H9V19H7V21Z" />
    </svg>
  }
}
