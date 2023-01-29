#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FruitCherriesOff)]
pub fn r#icon_fruit_cherries_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.1 21.5L2.4 1.7L1.1 3L8.7 10.6C8.3 11.6 7.9 12.5 7.6 13C5.6 13.2 4 14.9 4 17C4 19.2 5.8 21 8 21C9.1 21 10 20.6 10.7 19.9C10.3 19 10 18 10 17S10.3 15 10.7 14.1C10.3 13.7 9.7 13.4 9.2 13.2C9.4 12.8 9.6 12.3 9.9 11.7L12.8 14.6C12.3 15.3 12 16.1 12 17C12 19.2 13.8 21 16 21C16.9 21 17.7 20.7 18.3 20.2L20.8 22.7L22.1 21.5M8 15.5C7.2 15.5 6.5 16.2 6.5 17H5.5C5.5 15.6 6.6 14.5 8 14.5V15.5M14.5 17H13.5C13.5 16.5 13.6 16.1 13.8 15.7L14.6 16.5C14.5 16.7 14.5 16.8 14.5 17M20 16.8L16.2 13C18.2 13.1 19.9 14.8 20 16.8M11.3 8.1L10.1 6.9C10.7 5.1 11.2 3.3 11.2 2L12.6 1.9C12.7 2.3 12.7 2.8 12.8 3.4C13.4 3.3 15.5 3.3 17.8 5.1C20.5 7.1 19.8 10.9 19.8 10.9S17.4 11.3 14.7 9.2L13.8 8.3C14.1 9.5 14.5 10.7 14.9 11.7L12.6 9.4C12.3 8.4 12.1 7.4 11.9 6.4C11.7 7 11.5 7.6 11.3 8.1Z" />
    </svg>
  }
}
