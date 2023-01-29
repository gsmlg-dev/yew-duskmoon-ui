#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SignLanguageOutline)]
pub fn r#icon_sign_language_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7.5 4C7.9 3.6 8.5 3.6 8.9 4L11.8 7C11 7.4 10.6 7.6 10.2 8.2L7.5 5.5C7.1 5.1 7.1 4.4 7.5 4M6.3 7.2C6.7 6.8 7.3 6.8 7.7 7.2L9.6 9.2C9.4 9.8 9.3 10.5 9.4 11H8.6L6.3 8.6C5.9 8.2 5.9 7.6 6.3 7.2M18.2 14.5L12.5 9L12.2 9.1C11.5 9.5 11.2 10.4 11.5 11.1L12.4 13H4.4C3.9 13 3.4 13.5 3.4 14S3.9 15 4.4 15H10V16H3C2.5 16 2 16.5 2 17S2.5 18 3 18H10V19H4C3.5 19 3 19.5 3 20S3.5 21 4 21H10V22H5.5C5 22 4.5 22.5 4.5 23S5 24 5.5 24H16.5C17.9 24 19 22.9 19 21.5V16.3C19 15.6 18.7 14.9 18.2 14.5M17 21C17 21.5 16.5 22 16 22H12V15H14V13.2L16.7 15.7C16.9 15.9 17 16.2 17 16.4V21M22 11.3C22 12 21.7 12.6 21.2 13.1L20.3 13.9C20.1 13.6 19.9 13.3 19.6 13L19 12.4L19.7 11.7C19.9 11.5 20 11.2 20 11V7.3L18.7 8.5L17.3 7L15.4 9L12.7 6.6L8.9 2.6C8.5 2.2 8.5 1.6 8.9 1.2C9.3 .8 9.9 .8 10.3 1.2L15.1 6.3L15.8 5.6L12 1.6C11.6 1.2 11.6 .6 12 .2S13-.2 13.4 .2L18.9 6L19.6 4C20.1 3.4 21 3 21.7 3.2L22 3.3V11.3Z" />
    </svg>
  }
}
