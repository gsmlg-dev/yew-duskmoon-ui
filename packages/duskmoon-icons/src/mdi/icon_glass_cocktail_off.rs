#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_GlassCocktailOff)]
pub fn r#icon_glass_cocktail_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13.33 12.67L7.66 7L6.13 5.47L2.39 1.73L1.11 3L3 4.89V5L11 13V19H6V21H18V19.89L20.84 22.73L22.11 21.46L13.33 12.67M13 19V14.89L17.11 19H13M8.2 5L6.2 3H21V5L14.6 11.4L10.2 7H16.5L18.5 5H8.2Z" />
    </svg>
  }
}
