#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FaceMaskOutline)]
pub fn r#icon_face_mask_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 12L12.4 11.3C12.1 11.2 11.8 11.2 11.6 11.3L9 12V11L11.3 10.3C11.7 10.2 12.2 10.2 12.7 10.3L15 11V12M20.25 6C18.79 6 17.61 7.14 17.5 8.58L12.55 7.16C12.37 7.11 12.19 7.08 12 7.08C11.82 7.08 11.63 7.11 11.45 7.16L6.5 8.58C6.39 7.14 5.21 6 3.75 6C2.23 6 1 7.23 1 8.75V12.25C1 13.77 2.23 15 3.75 15H5.68C6.81 17.36 9.21 19 12 19S17.19 17.36 18.32 15H20.25C21.77 15 23 13.77 23 12.25V8.75C23 7.23 21.77 6 20.25 6M5 13.5H3.75C3.06 13.5 2.5 12.94 2.5 12.25V8.75C2.5 8.06 3.06 7.5 3.75 7.5S5 8.06 5 8.75V13.5M17 12C17 14.76 14.76 17 12 17S7 14.76 7 12L7 10.5L12 9.08H12L17 10.5V12M21.5 12.25C21.5 12.94 20.94 13.5 20.25 13.5H19V8.75C19 8.06 19.56 7.5 20.25 7.5S21.5 8.06 21.5 8.75V12.25Z" />
    </svg>
  }
}
