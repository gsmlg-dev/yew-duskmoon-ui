#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Fuse)]
pub fn r#icon_fuse(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8,7V17H15V7H8M11.16,16V12.87H9.41L11.91,8V11.14H13.59L11.16,16M16,2V6H7V2A1,1 0 0,1 8,1H15A1,1 0 0,1 16,2M16,18V22A1,1 0 0,1 15,23H8A1,1 0 0,1 7,22V18H16Z" />
    </svg>
  }
}
