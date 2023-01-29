#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_NetworkStrength3Alert)]
pub fn r#icon_network_strength_3_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 1L1 21H17V19H16V8.8L19 5.8V9H21M19 11V17H21V11M19 19V21H21V19" />
    </svg>
  }
}
