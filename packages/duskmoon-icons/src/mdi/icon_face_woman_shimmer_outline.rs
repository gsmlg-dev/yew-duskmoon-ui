#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FaceWomanShimmerOutline)]
pub fn r#icon_face_woman_shimmer_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.5 1L18.41 3.41L16 4.5L18.41 5.59L19.5 8L20.6 5.59L23 4.5L20.6 3.41M12 2C6.5 2 2 6.5 2 12V22H22V12C22 10.53 21.67 9.13 21.1 7.87L19.86 10.57C19.95 11.04 20 11.5 20 12C20 16.43 16.43 20 12 20C7.57 20 4 16.43 4 12C4 11.95 4 11.91 4 11.86C6.61 10.89 8.69 8.88 9.74 6.31C11.61 8.61 14.44 10 17.5 10C17.94 10 18.39 9.97 18.83 9.91L17.96 8C17.81 8 17.65 8 17.5 8C14.68 8 12.1 6.5 10.66 4.12C11.1 4.05 11.54 4 12 4C12.5 4 12.96 4.05 13.42 4.13L16.13 2.91C14.87 2.33 13.47 2 12 2M8.09 5C7.46 6.91 6.15 8.5 4.41 9.5C5.04 7.57 6.37 6 8.09 5M9 11.75C8.31 11.75 7.75 12.31 7.75 13C7.75 13.69 8.31 14.25 9 14.25C9.69 14.25 10.25 13.69 10.25 13C10.25 12.31 9.69 11.75 9 11.75M15 11.75C14.31 11.75 13.75 12.31 13.75 13C13.75 13.69 14.31 14.25 15 14.25C15.69 14.25 16.25 13.69 16.25 13C16.25 12.31 15.69 11.75 15 11.75M4 17.97C4.58 18.74 5.26 19.42 6.03 20H4M20 17.97V20H17.97C18.74 19.42 19.42 18.74 20 17.97Z" />
    </svg>
  }
}
