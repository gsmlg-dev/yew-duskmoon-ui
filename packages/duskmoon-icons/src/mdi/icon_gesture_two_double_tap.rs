#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_GestureTwoDoubleTap)]
pub fn r#icon_gesture_two_double_tap(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19,15.14V21.5C18.97,22.32 18.32,22.97 17.5,23H11C10.62,23 10.26,22.85 10,22.57L5.1,18.37L5.84,17.6C6.03,17.39 6.3,17.28 6.58,17.28H6.8L10,19V9A1,1 0 0,1 11,8A1,1 0 0,1 12,9V7A1,1 0 0,1 13,6A1,1 0 0,1 14,7V12L18.15,13.84C18.66,14.07 19,14.58 19,15.14M13,3A4,4 0 0,1 17,7C17,8.5 16.2,9.77 15,10.46V9.24C15.61,8.69 16,7.89 16,7A3,3 0 0,0 13,4C11.65,4 10.5,4.9 10.13,6.13C8.9,6.5 8,7.65 8,9C8,9.89 8.39,10.69 9,11.24V12.46C7.8,11.77 7,10.5 7,9C7,7.38 7.97,6 9.35,5.35C10,3.97 11.38,3 13,3M13,1A6,6 0 0,1 19,7C19,9.06 17.96,10.88 16.38,11.96L15.26,11.46C16.89,10.64 18,8.95 18,7A5,5 0 0,0 13,2C11.11,2 9.46,3.05 8.61,4.61C7.05,5.46 6,7.11 6,9C6,11.05 7.23,12.81 9,13.58V14.66C6.67,13.83 5,11.61 5,9C5,6.83 6.15,4.93 7.88,3.88C8.93,2.15 10.83,1 13,1Z" />
    </svg>
  }
}
