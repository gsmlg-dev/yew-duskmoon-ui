#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ChartAreasplineVariant)]
pub fn r#icon_chart_areaspline_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22,21H2V3H4V15.54L9.5,6L16,9.78L20.24,2.45L21.97,3.45L22,21Z" />
    </svg>
  }
}
