#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ChartSankey)]
pub fn r#icon_chart_sankey(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 5H4V2H2V22H22V20H4V9C8.09 9 10.13 11 12.29 13.21S17.09 18 22 18V16C17.91 16 15.87 14 13.71 11.79S8.91 7 4 7H22Z" />
    </svg>
  }
}
