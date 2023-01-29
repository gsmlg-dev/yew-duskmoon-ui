#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Vhs)]
pub fn r#icon_vhs(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4,6A2,2 0 0,0 2,8V16A2,2 0 0,0 4,18H20A2,2 0 0,0 22,16V8A2,2 0 0,0 20,6H4M4.54,10H7V14H4.54C4.19,13.39 4,12.7 4,12C4,11.3 4.19,10.61 4.54,10M9,10H15V14H9V10M17,10H19.46C19.81,10.61 20,11.3 20,12C20,12.7 19.81,13.39 19.46,14H17V10Z" />
    </svg>
  }
}
