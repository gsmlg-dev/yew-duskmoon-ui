#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_GoogleEarth)]
pub fn r#icon_google_earth(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,12.14C11.09,10.77 10.14,9.78 9.14,9.19C8.14,8.59 7.27,8.38 6.5,8.55C5.77,8.73 5.14,9.14 4.64,9.8C4.2,10.39 4,11.06 4,11.81V12C4,12.78 4.11,13.58 4.36,14.39C4.45,14.64 4.5,14.64 4.55,14.39C4.67,13.77 4.96,13.31 5.41,13.03C5.87,12.75 6.47,12.76 7.22,13.05C7.97,13.35 8.7,14 9.42,14.95C10.7,16.67 12.2,17.72 13.92,18.09C16.14,18.41 17.81,17.7 18.94,16C19.25,15.39 19.5,14.86 19.64,14.39C19.73,14.08 19.69,14.05 19.5,14.3C19.03,14.92 18.4,15.33 17.6,15.5C16.8,15.7 15.89,15.5 14.86,15C13.83,14.43 12.88,13.5 12,12.14M16.97,8.16C15.41,5.81 13.72,4.5 11.91,4.17C10.47,3.95 8.91,4.45 7.22,5.67C7,5.83 6.9,5.91 6.91,5.93C6.93,5.95 7.06,5.89 7.31,5.77C9.81,4.55 12.22,5.83 14.53,9.61C15.03,10.45 15.55,11.11 16.1,11.58C16.65,12.05 17.16,12.33 17.65,12.42C18.13,12.5 18.57,12.5 18.96,12.38C19.35,12.25 19.7,12.05 20,11.77C20,11.17 19.91,10.5 19.69,9.8C19.19,9.92 18.74,9.88 18.35,9.68C17.96,9.5 17.5,8.97 16.97,8.16M12,2C14.75,2 17.1,3 19.05,4.95C21,6.9 22,9.25 22,12C22,14.75 21,17.1 19.05,19.05C17.1,21 14.75,22 12,22C9.25,22 6.9,21 4.95,19.05C3,17.1 2,14.75 2,12C2,9.25 3,6.9 4.95,4.95C6.9,3 9.25,2 12,2Z" />
    </svg>
  }
}
