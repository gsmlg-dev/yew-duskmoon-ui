#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_OrderBoolAscending)]
pub fn r#icon_order_bool_ascending(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 3C3.79 3 2 4.79 2 7S3.79 11 6 11 10 9.21 10 7 8.21 3 6 3M6 9C4.9 9 4 8.1 4 7S4.9 5 6 5 8 5.9 8 7 7.1 9 6 9M6 13C3.79 13 2 14.79 2 17S3.79 21 6 21 10 19.21 10 17 8.21 13 6 13M12 5H22V7H12V5M12 19V17H22V19H12M12 11H22V13H12V11Z" />
    </svg>
  }
}
