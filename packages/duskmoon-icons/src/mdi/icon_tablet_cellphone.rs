#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TabletCellphone)]
pub fn r#icon_tablet_cellphone(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3,4H20A2,2 0 0,1 22,6V8H18V6H5V18H14V20H3A2,2 0 0,1 1,18V6A2,2 0 0,1 3,4M17,10H23A1,1 0 0,1 24,11V21A1,1 0 0,1 23,22H17A1,1 0 0,1 16,21V11A1,1 0 0,1 17,10M18,12V19H22V12H18Z" />
    </svg>
  }
}
