#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_KettlePourOver)]
pub fn r#icon_kettle_pour_over(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10 4C9.4 4 9 4.4 9 5V6H14V5C14 4.4 13.6 4 13 4H10M3 7L4.1 14.1C4.2 14.8 4.5 15.5 5.1 16.1C5.5 16.6 6.2 16.9 7 17C7 17.6 7.4 18 8 18H16C16.6 18 17 17.6 17 17L15 11H17L20.3 16L22 15L18.7 10L20 9.2L19 7.5L16.4 9H15V7H8V11L7.2 16C6.5 15.9 6.1 15.7 5.8 15.4C5.4 15 5.2 14.5 5.1 13.9L4 7H3M4 19V21H20V19H4Z" />
    </svg>
  }
}