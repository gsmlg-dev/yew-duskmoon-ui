#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ApplicationVariable)]
pub fn r#icon_application_variable(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 2H3C1.9 2 1 2.9 1 4V20C1 21.1 1.9 22 3 22H21C22.1 22 23 21.1 23 20V4C23 2.9 22.1 2 21 2M7.4 20C5.9 18.6 5 16.6 5 14.5S5.9 10.4 7.4 9L9 9.6C7.7 10.7 7 12.6 7 14.5S7.7 18.2 9 19.4L7.4 20M12.7 18L11.9 16L10.5 18H9L11.3 14.9L10 12H11.3L12.1 14L13.5 12H15L12.8 15L14.1 18H12.7M16.6 20L15 19.4C16.3 18.2 17 16.4 17 14.5S16.3 10.8 15 9.6L16.6 9C18.1 10.4 19 12.4 19 14.5C19 16.6 18.1 18.6 16.6 20M21 7H3V4H21V7Z" />
    </svg>
  }
}
