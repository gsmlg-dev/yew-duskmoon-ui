#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CarWindshield)]
pub fn r#icon_car_windshield(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.63 15.5L20.21 5.85A1 1 0 0 0 19.14 5.1C17.8 5.24 14.14 5.5 12 5.5S6.2 5.24 4.86 5.1A1 1 0 0 0 3.79 5.85L1.37 15.5A1.5 1.5 0 0 0 2.55 17.36A61.5 61.5 0 0 0 12 18A61.5 61.5 0 0 0 21.45 17.36A1.5 1.5 0 0 0 22.63 15.5Z" />
    </svg>
  }
}
