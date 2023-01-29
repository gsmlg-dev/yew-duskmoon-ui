#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AlarmPanel)]
pub fn r#icon_alarm_panel(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 3H4C2.9 3 2 3.9 2 5V19C2 20.1 2.9 21 4 21H20C21.1 21 22 20.1 22 19V5C22 3.9 21.1 3 20 3M8 19H5V17H8V19M8 16H5V14H8V16M8 13H5V11H8V13M13.5 19H10.5V17H13.5V19M13.5 16H10.5V14H13.5V16M13.5 13H10.5V11H13.5V13M19 19H16V17H19V19M19 16H16V14H19V16M19 13H16V11H19V13M19 9H5V5H19V9Z" />
    </svg>
  }
}
