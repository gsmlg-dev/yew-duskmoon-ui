#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Numeric10BoxMultipleOutline)]
pub fn r#icon_numeric_10_box_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 17V3H7V17H21M21 1C22.11 1 23 1.9 23 3V17C23 18.11 22.11 19 21 19H7C5.9 19 5 18.11 5 17V3C5 1.9 5.9 1 7 1H21M3 5V21H19V23H3C1.9 23 1 22.11 1 21V5H3M12 5H8V7H10V15H12V5M18 5H16C14.9 5 14 5.9 14 7V13C14 14.11 14.9 15 16 15H18C19.11 15 20 14.11 20 13V7C20 5.9 19.11 5 18 5M18 13H16V7H18V13Z" />
    </svg>
  }
}
