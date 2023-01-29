#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ConeOff)]
pub fn r#icon_cone_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10.5 7.31L9.06 5.86L11.15 2.47C11.34 2.16 11.67 2 12 2C12.33 2 12.66 2.16 12.85 2.5L21.62 16.68H21.62C21.91 17.15 22 17.61 22 18C22 18.25 21.96 18.5 21.9 18.7L17.73 14.53C17.81 14.55 17.9 14.56 18 14.58L12 4.9L10.5 7.31M22.11 21.46L20.84 22.73L19.17 21.06C17.03 21.83 14.11 22 12 22C3.3 22 2 19.5 2 18C2 17.5 2.15 17.04 2.4 16.65H2.38L7.11 9L1.11 3L2.39 1.73L22.11 21.46M6 14.6C7.96 14.12 10.25 14 12 14L12.11 14L8.56 10.45L6 14.6M17.55 19.44L14.19 16.08C13.5 16.03 12.76 16 12 16C7.58 16 4 16.9 4 18S7.58 20 12 20C14.16 20 16.11 19.79 17.55 19.44Z" />
    </svg>
  }
}
