#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ChiliMild)]
pub fn r#icon_chili_mild(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16 10V22C16 22 8 20 8 11V10C8 9.27 8.4 8.63 9 8.28L10.25 9L12 8L13.75 9L15 8.28C15.6 8.63 16 9.27 16 10M12 6.5L13.75 7.5L15.27 6.63C14.72 5.66 13.91 4.94 12.97 4.65C12.79 3.16 11.54 2 10 2V4C10.44 4 10.8 4.29 10.94 4.69C10.03 5 9.26 5.7 8.73 6.63L10.25 7.5L12 6.5Z" />
    </svg>
  }
}
