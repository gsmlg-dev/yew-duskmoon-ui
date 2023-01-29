#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_IcePop)]
pub fn r#icon_ice_pop(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.15 14.96L9.95 18.65L5.85 9.65C7.21 9.13 8.12 7.82 8.15 6.36C8.14 5 7.36 3.76 6.15 3.15C6.54 2.8 7 2.5 7.45 2.25C9.71 1.25 12.37 2.23 13.45 4.46M13.15 18.36L14.75 21.86L17.45 20.65L15.85 17.15" />
    </svg>
  }
}
