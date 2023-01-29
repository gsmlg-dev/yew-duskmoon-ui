#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FilePowerpoint)]
pub fn r#icon_file_powerpoint(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.6,12.3H10.6V15.5H12.7C13.3,15.5 13.6,15.3 13.9,15C14.2,14.7 14.3,14.4 14.3,13.9C14.3,13.4 14.2,13.1 13.9,12.8C13.6,12.5 13.2,12.3 12.6,12.3M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M15.2,16C14.6,16.5 14.1,16.7 12.8,16.7H10.6V20H9V11H12.8C14.1,11 14.7,11.3 15.2,11.8C15.8,12.4 16,13 16,13.9C16,14.8 15.8,15.5 15.2,16M13,9V3.5L18.5,9H13Z" />
    </svg>
  }
}
