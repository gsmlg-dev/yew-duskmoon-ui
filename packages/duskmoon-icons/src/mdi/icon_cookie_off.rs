#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CookieOff)]
pub fn r#icon_cookie_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9.25 6.05L7.45 4.25C8.79 3.46 10.34 3 12 3C12 3 13 3 13 4V6H14C14 6 15 6 15 7V8H17C17 8 18 8 18 9V10H20C20 10 20.6 10 20.87 10.5C20.96 11 21 11.5 21 12C21 13.66 20.54 15.22 19.75 16.55L17.95 14.75C17.97 14.67 18 14.59 18 14.5C18 13.67 17.33 13 16.5 13C16.41 13 16.33 13.03 16.25 13.05L10.95 7.75C10.97 7.67 11 7.59 11 7.5C11 6.67 10.33 6 9.5 6C9.41 6 9.33 6.03 9.25 6.05M22.11 21.46L20.84 22.73L17.34 19.23C15.85 20.34 14 21 12 21C7.03 21 3 16.97 3 12C3 10 3.66 8.15 4.77 6.66L1.11 3L2.39 1.73L22.11 21.46M8 11.5C8 10.67 7.33 10 6.5 10S5 10.67 5 11.5 5.67 13 6.5 13 8 12.33 8 11.5M10 12.5C10 13.33 10.67 14 11.5 14C11.68 14 11.85 13.96 12 13.9L10.1 12C10.04 12.15 10 12.32 10 12.5M12.5 17.5C12.5 16.67 11.83 16 11 16S9.5 16.67 9.5 17.5 10.17 19 11 19 12.5 18.33 12.5 17.5Z" />
    </svg>
  }
}
