#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CookieLock)]
pub fn r#icon_cookie_lock(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.5 11C19.42 11 20.27 11.29 21 11.77C21 11.35 20.95 10.92 20.87 10.5C20.6 10 20 10 20 10H18V9C18 8 17 8 17 8H15V7C15 6 14 6 14 6H13V4C13 3 12 3 12 3C7.03 3 3 7.03 3 12S7.03 21 12 21C12.34 21 12.67 21 13 20.94V17C13 16.08 13.42 15.26 14.07 14.71C14.45 12.6 16.29 11 18.5 11M6.5 13C5.67 13 5 12.33 5 11.5S5.67 10 6.5 10 8 10.67 8 11.5 7.33 13 6.5 13M8 7.5C8 6.67 8.67 6 9.5 6S11 6.67 11 7.5 10.33 9 9.5 9 8 8.33 8 7.5M11 19C10.17 19 9.5 18.33 9.5 17.5S10.17 16 11 16 12.5 16.67 12.5 17.5 11.83 19 11 19M11.5 14C10.67 14 10 13.33 10 12.5S10.67 11 11.5 11 13 11.67 13 12.5 12.33 14 11.5 14M21 16V15.5C21 14.12 19.88 13 18.5 13S16 14.12 16 15.5V16C15.45 16 15 16.45 15 17V21C15 21.55 15.45 22 16 22H21C21.55 22 22 21.55 22 21V17C22 16.45 21.55 16 21 16M20 16H17V15.5C17 14.67 17.67 14 18.5 14S20 14.67 20 15.5V16Z" />
    </svg>
  }
}
