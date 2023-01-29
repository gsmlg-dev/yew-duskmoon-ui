#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_HomeAlertOutline)]
pub fn r#icon_home_alert_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 5.7L17 10.2V18H7V10.2L12 5.7M19 20V12H22L12 3L2 12H5V20M13 8H11V13H13V8M13 15H11V17H13V15" />
    </svg>
  }
}
