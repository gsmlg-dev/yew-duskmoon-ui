#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_BriefcasePlusOutline)]
pub fn r#icon_briefcase_plus_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,2H14A2,2 0 0,1 16,4V6H20A2,2 0 0,1 22,8V13.53C21.42,13 20.75,12.6 20,12.34V8H4V19H12.08C12.2,19.72 12.45,20.39 12.8,21H4A2,2 0 0,1 2,19V8A2,2 0 0,1 4,6H8V4A2,2 0 0,1 10,2M14,6V4H10V6H14M14,17H17V14H19V17H22V19H19V22H17V19H14V17Z" />
    </svg>
  }
}
