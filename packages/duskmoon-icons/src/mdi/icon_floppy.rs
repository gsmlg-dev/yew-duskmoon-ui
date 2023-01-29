#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Floppy)]
pub fn r#icon_floppy(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5,3A2,2 0 0,0 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V5.5L18.5,3H17V9A1,1 0 0,1 16,10H8A1,1 0 0,1 7,9V3H5M12,4V9H15V4H12M7,12H17A1,1 0 0,1 18,13V19H6V13A1,1 0 0,1 7,12Z" />
    </svg>
  }
}
