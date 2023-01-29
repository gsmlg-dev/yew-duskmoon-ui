#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Blender)]
pub fn r#icon_blender(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.13 15.13L18 3H14V2H10V3H5C3.9 3 3 3.9 3 5V9C3 10.1 3.9 11 5 11H7.23L7.87 15.13C6.74 16.05 6 17.43 6 19V20C6 21.1 6.9 22 8 22H16C17.1 22 18 21.1 18 20V19C18 17.43 17.26 16.05 16.13 15.13M5 9V5H6.31L6.93 9H5M12 19C11.45 19 11 18.55 11 18S11.45 17 12 17 13 17.45 13 18 12.55 19 12 19M14.29 14H9.72L8.33 5H15.67L14.29 14Z" />
    </svg>
  }
}
