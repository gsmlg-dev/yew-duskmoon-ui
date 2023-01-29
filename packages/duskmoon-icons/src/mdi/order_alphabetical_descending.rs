#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_OrderAlphabeticalDescending)]
pub fn r#icon_order_alphabetical_descending(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7 13H5C3.9 13 3 13.9 3 15V21H5V19H7V21H9V15C9 13.9 8.11 13 7 13M7 17H5V15H7M9 3V5L5.67 9H9V11H3V9L6.33 5H3V3M12 5H22V7H12M12 19V17H22V19M12 11H22V13H12Z" />
    </svg>
  }
}
