#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_HumanDolly)]
pub fn r#icon_human_dolly(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.78 21.84C15.77 22.27 14.59 21.8 14.16 20.78C13.73 19.77 14.2 18.59 15.22 18.16C16.23 17.73 17.41 18.2 17.84 19.22C18.27 20.23 17.8 21.41 16.78 21.84M7.62 6C8.73 6 9.62 5.11 9.62 4C9.62 2.9 8.73 2 7.62 2C6.5 2 5.62 2.9 5.62 4C5.62 5.11 6.5 6 7.62 6M22.05 16.34L18.2 18C18.42 18.22 18.62 18.5 18.76 18.82C18.9 19.15 18.96 19.5 19 19.82L22.83 18.18L22.05 16.34M10.16 8.78L10.9 10.59C10.66 10.5 10.44 10.38 10.26 10.26C9.66 9.87 9.22 9.38 8.93 8.8L8.19 7.23C8 6.88 7.77 6.62 7.45 6.44C7.16 6.27 6.85 6.18 6.53 6.18C6.21 6.18 5.91 6.26 5.62 6.4C4.22 7.5 3.87 9.54 3.87 9.54L3.53 11.11C3.44 11.63 3.39 12.15 3.39 12.68V17.64L1 20.87L2.5 22L5.27 18.25L5.44 15L7.12 17.34V22H8.97V15.94L7.12 13.33V12.68C7.12 12.24 7.12 11.81 7.23 11.39L7.58 10.19C7.96 10.74 8.42 11.22 8.97 11.63C9.42 11.97 10.68 12.57 11.87 12.86L14 17.8C14.22 17.58 14.5 17.38 14.83 17.24C15.15 17.1 15.5 17.04 15.82 17L12 8L10.16 8.78M15.36 12.12L17.32 16.72L22.95 14.31L21 9.72" />
    </svg>
  }
}
