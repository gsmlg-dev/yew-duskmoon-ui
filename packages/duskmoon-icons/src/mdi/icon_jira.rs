#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Jira)]
pub fn r#icon_jira(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11.53,2C11.53,4.4 13.5,6.35 15.88,6.35H17.66V8.05C17.66,10.45 19.6,12.39 22,12.4V2.84A0.84,0.84 0 0,0 21.16,2H11.53M6.77,6.8C6.78,9.19 8.72,11.13 11.11,11.14H12.91V12.86C12.92,15.25 14.86,17.19 17.25,17.2V7.63C17.24,7.17 16.88,6.81 16.42,6.8H6.77M2,11.6C2,14 3.95,15.94 6.35,15.94H8.13V17.66C8.14,20.05 10.08,22 12.47,22V12.43A0.84,0.84 0 0,0 11.63,11.59L2,11.6Z" />
    </svg>
  }
}
