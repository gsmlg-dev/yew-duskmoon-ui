#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Jabber)]
pub fn r#icon_jabber(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5.23,5.72C4.61,5.55 5,3.19 5,3.17C5,3.19 12.58,-0.71 16.7,5.73C20.8,12.15 15.86,14.86 13.97,16.06C12.08,17.27 12.73,19.72 12.73,20.13C12.73,20.53 12.16,20.97 12,20.41C11.88,19.84 11.25,16.55 13.33,15.16C17.13,12.61 17.9,10.1 16,6.42C14.2,3 8.28,3.41 7.03,4.08C5.78,4.75 5.86,5.89 5.23,5.72M9.33,12.63L9.11,12.16C9.23,10.28 7.63,10.39 8.08,13.33C8.21,14.16 7.47,13.8 7.45,13.73C6.89,11.44 7.42,10.22 8.55,10.45C9.62,10.68 9.53,11.83 9.53,11.83C9.56,11.78 11,10.17 11.5,10.92C12.58,12.5 10.53,14.47 10.41,14.63C10.28,14.78 10.14,15.05 9.92,14.84C9.7,14.64 9.72,14.5 9.89,14.27C12.77,10 9.59,12.22 9.33,12.63M6.5,15.33V15.31C6.5,15.31 8,18.13 10.61,17.08C11,16.92 11.17,17.14 11.2,17.27C11.27,17.53 11.13,17.64 10.86,17.73C7.86,18.72 5.89,16.63 5.63,16.13C5.39,15.68 5.5,15.38 5.75,15.22C6,15.06 6.5,15.33 6.5,15.33M6,17.5C6,17.5 7.67,19.5 10.8,19C11.09,18.94 11.2,19.13 11.25,19.38C11.3,19.63 11.22,19.84 10.92,19.94C10.63,20.03 7.08,20.5 5.13,17.72C5,17.56 5.3,17.36 5.47,17.36C5.64,17.36 6.03,17.5 6,17.5M5.95,19.39C7.53,21.5 10.33,21.55 11.28,20.88C11.5,20.73 11.7,21.05 11.56,21.19C9.64,23.16 5.67,21 5.27,20.33C5.09,20.05 5.13,19.66 5.31,19.45C5.5,19.25 5.93,19.38 5.95,19.39Z" />
    </svg>
  }
}
