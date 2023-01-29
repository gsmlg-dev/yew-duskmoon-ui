#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(BS_EaselFill)]
pub fn r#icon_easel_fill(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8.473.337a.5.5 0 0 0-.946 0L6.954 2H2a1 1 0 0 0-1 1v7a1 1 0 0 0 1 1h1.85l-1.323 3.837a.5.5 0 1 0 .946.326L4.908 11H7.5v2.5a.5.5 0 0 0 1 0V11h2.592l1.435 4.163a.5.5 0 0 0 .946-.326L12.15 11H14a1 1 0 0 0 1-1V3a1 1 0 0 0-1-1H9.046L8.473.337z"/>
    </svg>
  }
}
