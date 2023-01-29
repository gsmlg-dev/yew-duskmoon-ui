#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_WifiRemove)]
pub fn r#icon_wifi_remove(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 12C9.97 12 8.1 12.67 6.6 13.8L4.8 11.4C6.81 9.89 9.3 9 12 9S17.19 9.89 19.2 11.4L17.92 13.1C17.55 13.17 17.18 13.27 16.84 13.41C15.44 12.5 13.78 12 12 12M21 9L22.8 6.6C19.79 4.34 16.05 3 12 3S4.21 4.34 1.2 6.6L3 9C5.5 7.12 8.62 6 12 6S18.5 7.12 21 9M12 15C10.65 15 9.4 15.45 8.4 16.2L12 21L13.04 19.61C13 19.41 13 19.21 13 19C13 17.66 13.44 16.43 14.19 15.43C13.5 15.16 12.77 15 12 15M21.12 15.46L19 17.59L16.88 15.47L15.47 16.88L17.59 19L15.47 21.12L16.88 22.54L19 20.41L21.12 22.54L22.54 21.12L20.41 19L22.54 16.88L21.12 15.46Z" />
    </svg>
  }
}
