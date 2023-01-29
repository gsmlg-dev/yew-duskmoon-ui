#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FuseOff)]
pub fn r#icon_fuse_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 11.8L10.2 7H15V11.8M16 6V2C16 1.45 15.55 1 15 1H8C7.45 1 7 1.45 7 2V3.8L9.2 6H16M2.39 1.73L1.11 3L8 9.89V17H15.11L20.84 22.73L22.11 21.46L2.39 1.73M7 22C7 22.55 7.45 23 8 23H15C15.55 23 16 22.55 16 22V18H7V22Z" />
    </svg>
  }
}
