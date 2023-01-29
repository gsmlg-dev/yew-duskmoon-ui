#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_LockSmart)]
pub fn r#icon_lock_smart(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,2A6,6 0 0,0 6,8V16A6,6 0 0,0 12,22A6,6 0 0,0 18,16V8A6,6 0 0,0 12,2M8,6H10V8H8V6M11,6H13V8H11V6M14,6H16V8H14V6M8,9H10V11H8V9M11,9H13V11H11V9M14,9H16V11H14V9M8,12H10V14H8V12M11,12H13V14H11V12M14,12H16V14H14V12M12,16A2,2 0 0,1 14,18A2,2 0 0,1 12,20A2,2 0 0,1 10,18A2,2 0 0,1 12,16Z" />
    </svg>
  }
}
