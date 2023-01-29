#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CalendarBadgeOutline)]
pub fn r#icon_calendar_badge_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19.5 16C17.6 16 16 17.6 16 19.5S17.6 23 19.5 23 23 21.4 23 19.5 21.4 16 19.5 16M14 19.5C14 19.33 14 19.17 14.03 19H5V9H19V14.03C19.17 14 19.33 14 19.5 14C20 14 20.5 14.08 21 14.21V5C21 3.9 20.11 3 19 3H18V1H16V3H8V1H6V3H5C3.89 3 3 3.9 3 5V19C3 20.11 3.9 21 5 21H14.21C14.08 20.5 14 20 14 19.5M5 5H19V7H5V5Z" />
    </svg>
  }
}
