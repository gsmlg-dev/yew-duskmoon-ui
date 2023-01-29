#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_DogSideOff)]
pub fn r#icon_dog_side_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 10L15 7L19 3V5L22 8L20 10L19 9L18 10M17 11L14 8L13 9H12.2L17 13.8V11M2.39 1.73L1.11 3L7.11 9H5L3 7L2 8L5 11V14L4 15V21H6V18L8 15H13.11L15 16.89V21H17V18.89L20.84 22.73L22.11 21.46L2.39 1.73Z" />
    </svg>
  }
}
