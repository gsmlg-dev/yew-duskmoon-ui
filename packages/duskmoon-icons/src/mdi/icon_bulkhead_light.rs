#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BulkheadLight)]
pub fn r#icon_bulkhead_light(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 2.09C13 2.06 13 2.03 13 2C13 1.45 12.55 1 12 1S11 1.45 11 2C11 2.03 11 2.06 11 2.09C8.19 2.56 6.03 5 6.03 7.97V16C6.03 19 8.19 21.44 11 21.91C11 21.94 11 21.97 11 22C11 22.55 11.45 23 12 23S13 22.55 13 22C13 21.97 13 21.94 13 21.91C15.81 21.44 17.97 19 17.97 16V7.97C18 5 15.81 2.56 13 2.09M16 8H15V5.4C15.6 6.09 16 7 16 8V8M14 19.44C13.41 19.79 12.73 20 12 20C11.27 20 10.59 19.79 10 19.44V16H14V19.44M10 15V9H14V15H10M9 15H8.03V9H9V15M10 4.56C10.59 4.21 11.27 4 12 4C12.73 4 13.41 4.21 14 4.56V8H10V4.56M15 9H16V15H15V9M9 5.4V8H8.03V7.97C8.03 7 8.4 6.09 9 5.4M8.03 16H9V18.6C8.4 17.91 8.03 17 8.03 16.03V16M15 18.6V16H16V16C16 17 15.6 17.91 15 18.6Z" />
    </svg>
  }
}
