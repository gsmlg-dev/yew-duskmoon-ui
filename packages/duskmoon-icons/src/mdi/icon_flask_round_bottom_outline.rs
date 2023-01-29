#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FlaskRoundBottomOutline)]
pub fn r#icon_flask_round_bottom_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 15C15 16.66 13.66 18 12 18C10.34 18 9 16.66 9 15V14H11V15C11 15.55 11.45 16 12 16C12.55 16 13 15.55 13 15V14H15V15M19 15C19 18.87 15.87 22 12 22C8.13 22 5 18.87 5 15C5 12.21 6.64 9.8 9 8.67V5C9 4.45 9.45 4 10 4H10.5L9.5 2H14.5L13.5 4H14C14.55 4 15 4.45 15 5V8.67C17.36 9.8 19 12.21 19 15M11 6V10.1C8.72 10.56 7 12.58 7 15C7 17.76 9.24 20 12 20C14.76 20 17 17.76 17 15C17 12.58 15.28 10.56 13 10.1V6H11Z" />
    </svg>
  }
}
