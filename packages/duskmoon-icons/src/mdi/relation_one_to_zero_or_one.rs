#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_RelationOneToZeroOrOne)]
pub fn r#icon_relation_one_to_zero_or_one(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 15V13H19V15H18.79A2.5 2.5 0 0 0 14.21 15H13V7H7V5H5V7H2V9H5V11H7V9H11V17H14.21A2.5 2.5 0 0 0 18.79 17H19V19H21V17H22V15M16.5 17A1 1 0 1 1 17.5 16A1 1 0 0 1 16.5 17Z" />
    </svg>
  }
}
