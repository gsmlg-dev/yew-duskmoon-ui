#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Basket)]
pub fn r#icon_basket(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5.5,21C4.72,21 4.04,20.55 3.71,19.9V19.9L1.1,10.44L1,10A1,1 0 0,1 2,9H6.58L11.18,2.43C11.36,2.17 11.66,2 12,2C12.34,2 12.65,2.17 12.83,2.44L17.42,9H22A1,1 0 0,1 23,10L22.96,10.29L20.29,19.9C19.96,20.55 19.28,21 18.5,21H5.5M12,4.74L9,9H15L12,4.74M12,13A2,2 0 0,0 10,15A2,2 0 0,0 12,17A2,2 0 0,0 14,15A2,2 0 0,0 12,13Z" />
    </svg>
  }
}
