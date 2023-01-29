#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_AlignHorizontalLeft)]
pub fn r#icon_align_horizontal_left(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4 22H2V2H4V22M22 7H6V10H22V7M16 14H6V17H16V14Z" />
    </svg>
  }
}
