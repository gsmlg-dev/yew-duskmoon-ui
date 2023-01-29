#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Cordova)]
pub fn r#icon_cordova(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.18,21.55H16.94L17.17,18.82H15.58L15.35,21.55H8.65L8.42,18.82H6.83L7.06,21.55H3.82L2,9.73L6.55,2.45H17.45L22,9.73L20.18,21.55M16.55,6.09H13.62L13.82,7.45H10.18L10.38,6.09H7.45L5.64,9.73L6.55,17H17.45L18.36,9.73L16.55,6.09M14.95,14.59C14.7,14.59 14.5,13.83 14.5,12.9C14.5,11.96 14.7,11.2 14.95,11.2C15.21,11.2 15.41,11.96 15.41,12.9C15.41,13.83 15.21,14.59 14.95,14.59M9.22,14.73C8.96,14.73 8.76,13.97 8.76,13.04C8.76,12.1 8.96,11.35 9.22,11.35C9.47,11.35 9.67,12.1 9.67,13.04C9.67,13.97 9.47,14.73 9.22,14.73Z" />
    </svg>
  }
}
