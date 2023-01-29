#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DecimalCommaDecrease)]
pub fn r#icon_decimal_comma_decrease(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 13V16H4L3 13A1 1 0 0 1 5 13M15 16V14L12 17L15 20V18H21V16M12 11A3 3 0 0 1 6 11V8A3 3 0 0 1 12 8M10 8A1 1 0 0 0 8 8V11A1 1 0 0 0 10 11Z" />
    </svg>
  }
}
