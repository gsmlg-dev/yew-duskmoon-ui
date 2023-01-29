#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ContainStart)]
pub fn r#icon_contain_start(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2,3H8V5H4V19H8V21H2V3M7,17V15H9V17H7M11,17V15H13V17H11M15,17V15H17V17H15Z" />
    </svg>
  }
}
