#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FlagOff)]
pub fn r#icon_flag_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.84 22.73L14.11 16H13L12.72 14.61L12.11 14H7V21H5V6.89L1.11 3L2.39 1.73L22.11 21.46L20.84 22.73M20 16V6H14.4L14 4H7.2L19.2 16H20" />
    </svg>
  }
}
