#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Numeric8BoxMultiple)]
pub fn r#icon_numeric_8_box_multiple(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13,11H15V13H13V11M13,7H15V9H13V7M3,5V21H19V23H3A2,2 0 0,1 1,21V5H3M13,15H15A2,2 0 0,0 17,13V11.5A1.5,1.5 0 0,0 15.5,10A1.5,1.5 0 0,0 17,8.5V7C17,5.89 16.1,5 15,5H13A2,2 0 0,0 11,7V8.5A1.5,1.5 0 0,0 12.5,10A1.5,1.5 0 0,0 11,11.5V13C11,14.11 11.9,15 13,15M21,1A2,2 0 0,1 23,3V17A2,2 0 0,1 21,19H7A2,2 0 0,1 5,17V3A2,2 0 0,1 7,1H21Z" />
    </svg>
  }
}
