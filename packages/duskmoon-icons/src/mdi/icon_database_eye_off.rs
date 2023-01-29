#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_DatabaseEyeOff)]
pub fn r#icon_database_eye_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.1 10.9L7.1 3.9C8.4 3.3 10.1 3 12 3C16.4 3 20 4.8 20 7C20 8.9 17.5 10.4 14.1 10.9M16.2 13H17C17.9 13 18.7 13.1 19.5 13.4C19.8 13 20 12.5 20 12V9C20 10.5 18.3 11.9 15.7 12.5L16.2 13M18.4 15.2L22.8 19.6C22.9 19.4 23 19.2 23.1 19.1C22.2 17.1 20.5 15.6 18.4 15.2M22.1 21.5L20.8 22.8L20.2 22.2C19.2 22.7 18.1 23.1 17 23.1C14.3 23.1 11.9 21.4 11 19.1C11.6 17.7 12.6 16.6 13.9 15.9L12.4 14.4C11.7 14.8 11.1 15.4 10.5 16C6.8 15.6 4 14 4 12V9C4 11.1 7.1 12.7 11.1 13L8.8 10.7C6 10 4 8.6 4 7C4 6.7 4.1 6.4 4.2 6.1L1.1 3L2.4 1.7L22.1 21.5M18.8 20.7L17.7 19.6C17.6 19.9 17.3 20 17 20C16.4 20 16 19.6 16 19C16 18.7 16.1 18.4 16.4 18.2L15.3 17.1C14.8 17.6 14.5 18.2 14.5 18.9C14.5 20.3 15.6 21.4 17 21.4C17.7 21.5 18.4 21.2 18.8 20.7M8.8 19L9.1 18.3C9.2 18.1 9.3 18 9.3 17.8C6.2 17.2 4 15.8 4 14V17C4 18.8 6.4 20.3 9.7 20.8C9.5 20.5 9.3 20.1 9.2 19.7L8.8 19Z" />
    </svg>
  }
}
