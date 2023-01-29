#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_GateNand)]
pub fn r#icon_gate_nand(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2,4V20H10C13.43,20 16.5,17.84 17.6,14.6C18,14.8 18.5,15 19,15A3,3 0 0,0 22,12A3,3 0 0,0 19,9C18.5,9 18.03,9.15 17.6,9.4C16.5,6.16 13.43,4 10,4H2M4,6H10A6,6 0 0,1 16,12A6,6 0 0,1 10,18H4V6M19,11C19.5,11 20,11.5 20,12C20,12.5 19.5,13 19,13A1,1 0 0,1 18,12C18,11.5 18.5,11 19,11Z" />
    </svg>
  }
}
