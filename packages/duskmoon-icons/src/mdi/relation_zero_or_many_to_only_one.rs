#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_RelationZeroOrManyToOnlyOne)]
pub fn r#icon_relation_zero_or_many_to_only_one(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 15V13H19V15H18V13H16V15H13V7H9.79A2.5 2.5 0 0 0 5.21 7H5L3 5H2V11H3L5 9H5.21A2.5 2.5 0 0 0 9.79 9H11V17H16V19H18V17H19V19H21V17H22V15M7.5 9A1 1 0 1 1 8.5 8A1 1 0 0 1 7.5 9Z" />
    </svg>
  }
}
