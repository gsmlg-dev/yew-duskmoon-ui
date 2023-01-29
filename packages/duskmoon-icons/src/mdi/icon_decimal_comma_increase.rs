#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DecimalCommaIncrease)]
pub fn r#icon_decimal_comma_increase(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 5A3 3 0 0 0 6 8V11A3 3 0 0 0 12 11V8A3 3 0 0 0 9 5M10 11A1 1 0 0 1 8 11V8A1 1 0 0 1 10 8M16 14A3 3 0 0 0 19 11V8A3 3 0 0 0 13 8V11A3 3 0 0 0 16 14M15 8A1 1 0 0 1 17 8V11A1 1 0 0 1 15 11M19 20V18H13V16H19V14L22 17M5 13V16H4L3 13A1 1 0 0 1 5 13Z" />
    </svg>
  }
}
