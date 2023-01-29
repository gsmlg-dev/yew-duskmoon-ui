#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Resistor)]
pub fn r#icon_resistor(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2,11H7L10.07,15.35L13.11,4L18,11H22V13H17L13.93,8.65L10.89,20L6,13H2V11Z" />
    </svg>
  }
}
