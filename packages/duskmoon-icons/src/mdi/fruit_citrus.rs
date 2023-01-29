#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FruitCitrus)]
pub fn r#icon_fruit_citrus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 15.9C6.9 15.7 5.1 14.9 4 13.6C3.2 13.6 2.4 13.1 2.1 12.3C1.8 11.5 2.1 10.7 2.8 10.2C2.9 7.2 5.6 4.1 9.7 2.7C13.8 1.3 18 2.1 20 4.4C20.8 4.4 21.6 4.9 21.9 5.7C22.2 6.5 21.9 7.3 21.2 7.8C21.2 8.6 21 9.4 20.6 10.2C19.4 9.5 18 9 16.5 9C12.6 9 9.3 12 9 15.9M22 16.5C22 19.5 19.5 22 16.5 22S11 19.5 11 16.5 13.5 11 16.5 11 22 13.5 22 16.5M13.3 18L15.6 16.5L13.3 15C13.1 15.5 13 16 13 16.5S13.1 17.5 13.3 18M16 17.4L13.9 18.8C14.4 19.4 15.2 19.8 16 20V17.4M16 13.1C15.2 13.2 14.4 13.6 13.9 14.3L16 15.7V13.1M17 15.6L19.1 14.2C18.6 13.6 17.8 13.2 17 13V15.6M19.1 18.8L17 17.4V19.9C17.8 19.8 18.6 19.4 19.1 18.8M20 16.5C20 16 19.9 15.5 19.7 15L17.4 16.5L19.7 18C19.9 17.5 20 17 20 16.5Z" />
    </svg>
  }
}