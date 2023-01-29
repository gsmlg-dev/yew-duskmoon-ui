#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AirballoonOutline)]
pub fn r#icon_airballoon_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11 23C9.9 23 9 22.1 9 21V19H15V21C15 22.1 14.1 23 13 23H11M12 3C12.28 3 12.55 3 12.81 3.05C13.42 4.22 14 6.26 14 9C14 11.1 13 16 13 16H11C11 16 10 11.1 10 9C10 6.26 10.58 4.22 11.19 3.05C11.45 3 11.72 3 12 3M12 1C11.29 1 10.61 1.09 9.95 1.26C8.78 2.83 8 5.71 8 9C8 11.28 8.38 13.37 9 16C9 17.1 9.9 18 11 18H13C14.1 18 15 17.1 15 16C15.62 13.37 16 11.28 16 9C16 5.71 15.22 2.83 14.05 1.26C13.39 1.09 12.71 1 12 1M4 8C4 11.18 5.85 15.92 8.54 17.21C8 16.21 7.61 14.67 7.34 13C6.55 11.53 6 9.62 6 8C6 6.66 6.44 5.67 7.47 4.8C7.73 3.67 8.09 2.65 8.54 1.79C5.85 3.08 4 4.82 4 8M15.46 1.79C15.91 2.65 16.27 3.67 16.53 4.8C17.56 5.67 18 6.66 18 8C18 9.62 17.45 11.53 16.66 13C16.39 14.67 16 16.21 15.46 17.21C18.15 15.92 20 11.18 20 8S18.15 3.08 15.46 1.79Z" />
    </svg>
  }
}
