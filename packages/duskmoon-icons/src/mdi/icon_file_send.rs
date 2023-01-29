#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FileSend)]
pub fn r#icon_file_send(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14,2H6C4.89,2 4,2.89 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M12.54,19.37V17.37H8.54V15.38H12.54V13.38L15.54,16.38L12.54,19.37M13,9V3.5L18.5,9H13Z" />
    </svg>
  }
}
