#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_GiftOpenOutline)]
pub fn r#icon_gift_open_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 10.87L19.26 9.28C19.5 9.07 19.73 8.8 19.9 8.5C20.73 7.07 20.24 5.23 18.8 4.4C17.94 3.9 16.93 3.9 16.08 4.26L16.09 4.25L15.21 4.64L15.1 3.68L15.09 3.69C15 2.78 14.47 1.9 13.61 1.4C12.17 .575 10.34 1.07 9.5 2.5C9.33 2.8 9.22 3.13 9.16 3.45L6.41 1.87C5.45 1.32 4.23 1.64 3.68 2.6L2.18 5.2C1.9 5.68 2.07 6.29 2.55 6.56L4.28 7.56L8.5 10H2V20C2 21.11 2.9 22 4 22H20C21.11 22 22 21.11 22 20V14.87L22.73 13.6C23.28 12.64 22.96 11.42 22 10.87M16.44 6.5C16.71 6 17.33 5.86 17.8 6.13C18.28 6.41 18.45 7 18.17 7.5C17.89 8 17.28 8.14 16.8 7.87C16.33 7.59 16.16 7 16.44 6.5M14.07 8.6L21 12.6L20 14.33L13.07 10.33L14.07 8.6M11 20H4V12H11V20M11.34 9.33L4.41 5.33L5.41 3.6L12.34 7.6L11.34 9.33M11.61 4.87C11.13 4.59 10.97 4 11.24 3.5C11.5 3 12.13 2.86 12.61 3.13C13.09 3.41 13.25 4 12.97 4.5C12.7 5 12.09 5.14 11.61 4.87M13 20V12.6L20 16.64V20H13Z" />
    </svg>
  }
}
