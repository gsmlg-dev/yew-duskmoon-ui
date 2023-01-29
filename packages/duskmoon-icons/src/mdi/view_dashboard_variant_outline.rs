#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ViewDashboardVariantOutline)]
pub fn r#icon_view_dashboard_variant_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2 5V19H22V5H2M20 12H16V7H20V12M14 10H10V7H14V10M10 12H14V17H10V12M4 7H8V17H4V7M16 17V14H20V17H16Z" />
    </svg>
  }
}
