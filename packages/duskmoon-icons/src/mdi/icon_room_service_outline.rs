#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RoomServiceOutline)]
pub fn r#icon_room_service_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,5A2,2 0 0,1 14,7C14,7.24 13.96,7.47 13.88,7.69C17.95,8.5 21,11.91 21,16H3C3,11.91 6.05,8.5 10.12,7.69C10.04,7.47 10,7.24 10,7A2,2 0 0,1 12,5M22,19H2V17H22V19M12,9.5C8.89,9.5 6.25,11.39 5.34,14H18.66C17.75,11.39 15.11,9.5 12,9.5Z" />
    </svg>
  }
}
