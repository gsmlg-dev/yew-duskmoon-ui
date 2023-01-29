#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_GarageAlertVariant)]
pub fn r#icon_garage_alert_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 9V20H18V11H2V20H0V9L10 5L20 9M17 12H3V14H17V12M17 15H3V17H17V15M22 15V10H24V15H22M22 19V17H24V19H22Z" />
    </svg>
  }
}
