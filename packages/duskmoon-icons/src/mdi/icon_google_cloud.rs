#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_GoogleCloud)]
pub fn r#icon_google_cloud(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M23 14.75C23 18.2 20.2 21 16.75 21H7.25C3.8 21 1 18.2 1 14.75C1 12.61 2.08 10.72 3.71 9.6C4.58 5.82 7.96 3 12 3C16.04 3 19.42 5.82 20.29 9.6C21.93 10.72 23 12.61 23 14.75M16.63 17C17.94 17 19 15.94 19 14.63C19 13.35 18 12.3 16.72 12.25L16.75 11.75C16.75 9.13 14.62 7 12 7C10.58 7 9.3 7.62 8.43 8.61C9.93 8.9 11.23 9.72 12.14 10.86L9.5 13.5C9.08 12.77 8.29 12.25 7.38 12.25C6.06 12.25 5 13.31 5 14.63C5 15.9 6 16.93 7.25 17V17H16.63Z" />
    </svg>
  }
}
