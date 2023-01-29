#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_HomeAlert)]
pub fn r#icon_home_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 3L2 12H5V20H19V12H22L12 3M13 18H11V16H13V18M13 14H11V8H13V14Z" />
    </svg>
  }
}
