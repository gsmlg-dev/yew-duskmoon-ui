#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Numeric3BoxMultipleOutline)]
pub fn r#icon_numeric_3_box_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17,13V11.5A1.5,1.5 0 0,0 15.5,10A1.5,1.5 0 0,0 17,8.5V7C17,5.89 16.1,5 15,5H11V7H15V9H13V11H15V13H11V15H15A2,2 0 0,0 17,13M3,5H1V21A2,2 0 0,0 3,23H19V21H3M21,17H7V3H21M21,1H7A2,2 0 0,0 5,3V17A2,2 0 0,0 7,19H21A2,2 0 0,0 23,17V3A2,2 0 0,0 21,1Z" />
    </svg>
  }
}
