#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ScatterPlot)]
pub fn r#icon_scatter_plot(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7,11A3,3 0 0,1 10,14A3,3 0 0,1 7,17A3,3 0 0,1 4,14A3,3 0 0,1 7,11M11,3A3,3 0 0,1 14,6A3,3 0 0,1 11,9A3,3 0 0,1 8,6A3,3 0 0,1 11,3M16.6,14.6C18.25,14.6 19.6,15.94 19.6,17.6A3,3 0 0,1 16.6,20.6C14.94,20.6 13.6,19.25 13.6,17.6A3,3 0 0,1 16.6,14.6Z" />
    </svg>
  }
}
