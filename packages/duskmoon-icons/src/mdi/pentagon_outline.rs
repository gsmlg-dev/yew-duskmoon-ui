#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_PentagonOutline)]
pub fn r#icon_pentagon_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,5L19.6,10.5L16.7,19.4H7.3L4.4,10.5L12,5M12,2.5L2,9.8L5.8,21.5H18.1L22,9.8L12,2.5Z" />
    </svg>
  }
}
