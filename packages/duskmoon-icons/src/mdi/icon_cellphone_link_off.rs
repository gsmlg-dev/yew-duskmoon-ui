#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CellphoneLinkOff)]
pub fn r#icon_cellphone_link_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M23,8H17A1,1 0 0,0 16,9V13.18L18,15.18V10H22V17H19.82L22.82,20H23A1,1 0 0,0 24,19V9A1,1 0 0,0 23,8M4,6.27L14.73,17H4V6.27M1.92,1.65L0.65,2.92L2.47,4.74C2.18,5.08 2,5.5 2,6V17H0V20H17.73L20.08,22.35L21.35,21.08L3.89,3.62L1.92,1.65M22,6V4H6.82L8.82,6H22Z" />
    </svg>
  }
}
