#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AccountChildCircle)]
pub fn r#icon_account_child_circle(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,12A1.5,1.5 0 0,1 13.5,13.5A1.5,1.5 0 0,1 12,15A1.5,1.5 0 0,1 10.5,13.5A1.5,1.5 0 0,1 12,12M12,2A10,10 0 0,1 22,12A10,10 0 0,1 12,22A10,10 0 0,1 2,12A10,10 0 0,1 12,2M12,16C12.72,16 13.4,16.15 14.04,16.5C14.68,16.8 15,17.2 15,17.67V19.41C16.34,18.81 17,18.08 17,17.2V12.8C17,12 16.5,11.35 15.45,10.8C14.4,10.26 13.25,10 12,10C10.75,10 9.6,10.26 8.55,10.8C7.5,11.35 7,12 7,12.8V17.2C7,18 7.53,18.69 8.63,19.22C9.72,19.75 10.84,20 12,20L13,19.92V17.91L12,18C11,18 10,17.8 9.05,17.39C9.17,17 9.53,16.69 10.13,16.41C10.72,16.13 11.34,16 12,16M12,4A2.5,2.5 0 0,0 9.5,6.5A2.5,2.5 0 0,0 12,9A2.5,2.5 0 0,0 14.5,6.5A2.5,2.5 0 0,0 12,4Z" />
    </svg>
  }
}
