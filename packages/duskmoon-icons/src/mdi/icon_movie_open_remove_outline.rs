#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_MovieOpenRemoveOutline)]
pub fn r#icon_movie_open_remove_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.65 6.5L16.91 2.96L20.84 2.18L21.62 6.1L19.65 6.5M16.71 7.07L13.97 3.54L12 3.93L14.75 7.46L16.71 7.07M4.16 5.5L3.18 5.69C2.1 5.9 1.39 6.96 1.61 8.04L2 10L6.9 9.03L4.16 5.5M11.81 8.05L9.07 4.5L7.1 4.91L9.85 8.44L11.81 8.05M4 20V12H20V13.09C20.72 13.21 21.39 13.46 22 13.81V10H2V20C2 21.11 2.9 22 4 22H13.81C13.46 21.39 13.22 20.72 13.09 20H4M22.54 16.88L21.12 15.47L19 17.59L16.88 15.47L15.47 16.88L17.59 19L15.47 21.12L16.88 22.54L19 20.41L21.12 22.54L22.54 21.12L20.41 19L22.54 16.88Z" />
    </svg>
  }
}
