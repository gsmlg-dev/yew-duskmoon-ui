#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Muffin)]
pub fn r#icon_muffin(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16,5C16,5 15,2 12,2C9,2 8,5 8,5C6,5 4,7 4,9C1.3,9 1.3,13 4,13H20C22.7,13 22.7,9 20,9C20,7 18,5 16,5M5,15L7,22H9L8,15H5M10,15L11,22H13L14,15H10M16,15L15,22H17L19,15H16Z" />
    </svg>
  }
}
