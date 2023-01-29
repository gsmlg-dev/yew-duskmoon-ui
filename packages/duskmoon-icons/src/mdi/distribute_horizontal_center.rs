#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_DistributeHorizontalCenter)]
pub fn r#icon_distribute_horizontal_center(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8 2V5H10V19H8V22H6V19H4V5H6V2H8M16 2V7H14V17H16V22H18V17H20V7H18V2H16Z" />
    </svg>
  }
}
