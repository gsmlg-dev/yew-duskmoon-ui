#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Numeric9PlusBoxMultipleOutline)]
pub fn r#icon_numeric_9_plus_box_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21,9H19V7H17V9H15V11H17V13H19V11H21V17H7V3H21M21,1H7A2,2 0 0,0 5,3V17A2,2 0 0,0 7,19H21A2,2 0 0,0 23,17V3A2,2 0 0,0 21,1M11,9V8H12V9M14,12V8C14,6.89 13.1,6 12,6H11A2,2 0 0,0 9,8V9C9,10.11 9.9,11 11,11H12V12H9V14H12A2,2 0 0,0 14,12M3,5H1V21A2,2 0 0,0 3,23H19V21H3V5Z" />
    </svg>
  }
}
