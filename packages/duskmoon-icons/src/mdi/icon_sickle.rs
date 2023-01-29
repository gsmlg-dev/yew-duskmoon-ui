#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Sickle)]
pub fn r#icon_sickle(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.3 7.2C17.5 4.7 14.9 3 12 2C26.2 10.5 15.4 22.9 8.5 15.5L5.9 16L2.5 19.4C1.9 20 1.9 21 2.5 21.5C3.1 22.1 4.1 22.1 4.6 21.5L7.8 18.3C15.3 24.3 25 15 19.3 7.2Z" />
    </svg>
  }
}
