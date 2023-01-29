#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Numeric8CircleOutline)]
pub fn r#icon_numeric_8_circle_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11,13V15H13V13H11M11,9V11H13V9H11M11,17A2,2 0 0,1 9,15V13.5A1.5,1.5 0 0,1 10.5,12A1.5,1.5 0 0,1 9,10.5V9A2,2 0 0,1 11,7H13A2,2 0 0,1 15,9V10.5A1.5,1.5 0 0,1 13.5,12A1.5,1.5 0 0,1 15,13.5V15A2,2 0 0,1 13,17H11M12,2A10,10 0 0,1 22,12A10,10 0 0,1 12,22A10,10 0 0,1 2,12A10,10 0 0,1 12,2M12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20A8,8 0 0,0 20,12A8,8 0 0,0 12,4Z" />
    </svg>
  }
}
