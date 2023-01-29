#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FoodCroissant)]
pub fn r#icon_food_croissant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22,19L19,17L22,15V19M15,15L19,9L22,13L18,16L15,15M5,17L2,19V15L5,17M9,15L6,16L2,13L5,9L9,15M14,6L18,8L13,15H11L6,8L10,6H14Z" />
    </svg>
  }
}
