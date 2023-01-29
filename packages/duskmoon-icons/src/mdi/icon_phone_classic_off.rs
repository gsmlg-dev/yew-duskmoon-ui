#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PhoneClassicOff)]
pub fn r#icon_phone_classic_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 3C16.53 3 20.65 4.78 23.7 7.67C23.88 7.85 24 8.09 24 8.37C24 8.65 23.89 8.9 23.71 9.08L21.23 11.56C21.05 11.74 20.8 11.85 20.5 11.85C20.25 11.85 20 11.75 19.82 11.57C19 10.84 18.13 10.21 17.15 9.72C16.82 9.56 16.59 9.21 16.59 8.82V5.72C15.14 5.25 13.59 5 12 5C10.44 5 8.93 5.24 7.5 5.69L5.94 4.11C7.82 3.4 9.86 3 12 3M9 7H11V9H13V7H15V10C15 10 21 15 21 18V19.18L9 7.18V7M1 4.27L2.28 3L21.5 22.22L20.23 23.5L18.73 22H3V18C3 15.86 6.05 12.71 7.8 11.07L6.59 9.86C5.71 10.33 4.9 10.9 4.18 11.58C4 11.75 3.75 11.86 3.5 11.86C3.2 11.86 2.95 11.75 2.77 11.57L.29 9.09C.11 8.91 0 8.66 0 8.38C0 8.1 .11 7.85 .29 7.67C.996 7 2.58 5.85 2.58 5.85L1 4.27M8 16C8 18.21 9.79 20 12 20C13.29 20 14.44 19.39 15.17 18.44L14.1 17.36C13.65 18.05 12.88 18.5 12 18.5C10.62 18.5 9.5 17.38 9.5 16C9.5 15.12 9.95 14.35 10.64 13.91L9.56 12.83C8.61 13.56 8 14.71 8 16Z" />
    </svg>
  }
}
