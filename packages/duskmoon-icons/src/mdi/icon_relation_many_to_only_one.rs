#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RelationManyToOnlyOne)]
pub fn r#icon_relation_many_to_only_one(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 15V17H21V19H19V17H18V19H16V17H11V9H5L3 11H2V5H3L5 7H13V15H16V13H18V15H19V13H21V15Z" />
    </svg>
  }
}
