#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CoachLamp)]
pub fn r#icon_coach_lamp(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16 5L15 2H13L12 5L6 8H8L8.6 11H4V7H2V17H4V13H9L10 18L12 20L13 22H15L16 20L18 18L20 8H22M16.16 17H11.84L10 8H18Z" />
    </svg>
  }
}
