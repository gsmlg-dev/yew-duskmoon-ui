#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ArrowTopRightThick)]
pub fn r#icon_arrow_top_right_thick(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8.5,18.31L5.69,15.5L12.06,9.12H7.11V5.69H18.31V16.89H14.89V11.94L8.5,18.31Z" />
    </svg>
  }
}
