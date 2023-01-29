#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ArrowBottomRightThick)]
pub fn r#icon_arrow_bottom_right_thick(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.89,12.06V7.11H18.31V18.31H7.11V14.89H12.06L5.69,8.5L8.5,5.69L14.89,12.06Z" />
    </svg>
  }
}
