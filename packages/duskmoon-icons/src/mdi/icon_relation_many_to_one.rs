#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RelationManyToOne)]
pub fn r#icon_relation_many_to_one(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 15V17H19V19H17V17H11V9H5L3 11H2V5H3L5 7H13V15H17V13H19V15Z" />
    </svg>
  }
}
