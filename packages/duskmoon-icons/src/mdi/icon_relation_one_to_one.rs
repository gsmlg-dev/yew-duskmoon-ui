#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RelationOneToOne)]
pub fn r#icon_relation_one_to_one(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 15V17H19V19H17V17H11V9H7V11H5V9H2V7H5V5H7V7H13V15H17V13H19V15Z" />
    </svg>
  }
}
