#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_SmartCard)]
pub fn r#icon_smart_card(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2 3H22A2.07 2.07 0 0 1 24 5V19A2.07 2.07 0 0 1 22 21H2A2.07 2.07 0 0 1 0 19V5A2.07 2.07 0 0 1 2 3M8 13.91C6 13.91 2 15 2 17V18H14V17C14 15 10 13.91 8 13.91M8 6A3 3 0 1 0 11 9A3 3 0 0 0 8 6M17 10V13H21V10H17" />
    </svg>
  }
}
