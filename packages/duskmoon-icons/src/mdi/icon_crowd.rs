#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Crowd)]
pub fn r#icon_crowd(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3.69 9.12C3.5 8.93 3.29 8.84 3.04 8.84C2.63 8.84 2.32 9.03 2.12 9.42S1.97 10.18 2.29 10.53C3.47 11.59 4.22 12.34 4.54 12.78C4.95 13.34 5.15 14.16 5.15 15.22C5.15 16.53 5.65 17.5 6.65 18.17C7.21 18.61 7.82 18.94 8.5 19.16L8.5 15.27C8.5 14.33 8.17 13.55 7.54 12.92M16.46 12.97C15.84 13.59 15.5 14.36 15.5 15.27L15.5 19.2C16.46 18.86 17.26 18.33 17.92 17.63C18.57 16.93 18.9 16.16 18.9 15.22C18.9 14.09 19.09 13.28 19.47 12.78C19.56 12.62 19.73 12.42 20 12.17C20.23 11.92 20.47 11.68 20.71 11.46C20.94 11.25 21.17 11.03 21.39 10.81L21.72 10.53C21.91 10.34 22 10.12 22 9.87C22 9.59 21.91 9.34 21.72 9.14C21.53 8.94 21.3 8.84 21 8.84S20.5 8.93 20.31 9.12M12 20C12.69 20 13.36 19.91 14 19.72L14 16.15C14 15.56 13.82 15.1 13.41 14.66C13 14.22 12.53 14 12 14C11.47 14 11 14.2 10.62 14.61C10.22 15 10 15.46 10 16.06L10 19.72C10.64 19.91 11.31 20 12 20M9 8.5C9 9.33 8.33 10 7.5 10S6 9.33 6 8.5 6.67 7 7.5 7 9 7.67 9 8.5M18 8.5C18 9.33 17.33 10 16.5 10C15.67 10 15 9.33 15 8.5S15.67 7 16.5 7C17.33 7 18 7.67 18 8.5M13.5 5.5C13.5 6.33 12.83 7 12 7S10.5 6.33 10.5 5.5 11.17 4 12 4 13.5 4.67 13.5 5.5M13.5 11C13.5 11.83 12.83 12.5 12 12.5S10.5 11.83 10.5 11 11.17 9.5 12 9.5 13.5 10.17 13.5 11Z" />
    </svg>
  }
}
