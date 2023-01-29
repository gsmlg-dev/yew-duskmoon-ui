#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FruitWatermelon)]
pub fn r#icon_fruit_watermelon(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.4 16.4C19.8 13 19.8 7.5 16.4 4.2L4.2 16.4C7.5 19.8 13 19.8 16.4 16.4M16 7C16.6 7 17 7.4 17 8C17 8.6 16.6 9 16 9S15 8.6 15 8C15 7.4 15.4 7 16 7M16 11C16.6 11 17 11.4 17 12C17 12.6 16.6 13 16 13S15 12.6 15 12C15 11.4 15.4 11 16 11M12 11C12.6 11 13 11.4 13 12C13 12.6 12.6 13 12 13S11 12.6 11 12C11 11.4 11.4 11 12 11M12 15C12.6 15 13 15.4 13 16C13 16.6 12.6 17 12 17S11 16.6 11 16C11 15.4 11.4 15 12 15M8 17C7.4 17 7 16.6 7 16C7 15.4 7.4 15 8 15S9 15.4 9 16C9 16.6 8.6 17 8 17M18.6 18.6C14 23.2 6.6 23.2 2 18.6L3.4 17.2C7.2 21 13.3 21 17.1 17.2C20.9 13.4 20.9 7.3 17.1 3.5L18.6 2C23.1 6.6 23.1 14 18.6 18.6Z" />
    </svg>
  }
}
