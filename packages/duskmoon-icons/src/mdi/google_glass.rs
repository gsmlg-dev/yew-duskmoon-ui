#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_GoogleGlass)]
pub fn r#icon_google_glass(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 2H4C2.9 2 2 2.9 2 4V20C2 21.1 2.9 22 4 22H20C21.1 22 22 21.1 22 20V4C22 2.9 21.1 2 20 2M16.75 18H15.25V8.9L6.72 18H4.66L15.45 6.5C15.66 6.26 16 6.19 16.27 6.3C16.56 6.42 16.75 6.69 16.75 7V18Z" />
    </svg>
  }
}
