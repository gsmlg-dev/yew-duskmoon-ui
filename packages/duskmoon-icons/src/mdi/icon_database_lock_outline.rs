#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_DatabaseLockOutline)]
pub fn r#icon_database_lock_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M6 12.45V9.64C7.47 10.47 9.61 11 12 11S16.53 10.47 18 9.64V11.05C18.17 11.03 18.33 11 18.5 11C19.03 11 19.53 11.1 20 11.26V7C20 4.79 16.42 3 12 3S4 4.79 4 7V17C4 19.21 7.59 21 12 21C12.34 21 12.67 21 13 20.97V18.95C12.68 19 12.35 19 12 19C8.13 19 6 17.5 6 17V14.77C7.61 15.55 9.72 16 12 16C12.41 16 12.81 15.97 13.21 15.94C13.4 15.46 13.68 15.03 14.07 14.7C14.13 14.39 14.23 14.09 14.34 13.8C13.6 13.93 12.81 14 12 14C9.58 14 7.3 13.4 6 12.45M12 5C15.87 5 18 6.5 18 7S15.87 9 12 9 6 7.5 6 7 8.13 5 12 5M21 16V15.5C21 14.12 19.88 13 18.5 13S16 14.12 16 15.5V16C15.45 16 15 16.45 15 17V21C15 21.55 15.45 22 16 22H21C21.55 22 22 21.55 22 21V17C22 16.45 21.55 16 21 16M20 16H17V15.5C17 14.67 17.67 14 18.5 14S20 14.67 20 15.5V16Z" />
    </svg>
  }
}
