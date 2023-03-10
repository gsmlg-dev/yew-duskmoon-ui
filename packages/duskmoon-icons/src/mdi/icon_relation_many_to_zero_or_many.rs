#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RelationManyToZeroOrMany)]
pub fn r#icon_relation_many_to_zero_or_many(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 13L19 15H18.79A2.5 2.5 0 0 0 14.21 15H13V7H5L3 5H2V11H3L5 9H11V17H14.21A2.5 2.5 0 0 0 18.79 17H19L21 19H22V13M16.5 17A1 1 0 1 1 17.5 16A1 1 0 0 1 16.5 17Z" />
    </svg>
  }
}
