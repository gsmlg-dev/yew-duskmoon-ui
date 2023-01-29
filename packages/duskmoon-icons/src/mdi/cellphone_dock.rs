#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CellphoneDock)]
pub fn r#icon_cellphone_dock(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16,15H8V5H16M16,1H8C6.89,1 6,1.89 6,3V17A2,2 0 0,0 8,19H16A2,2 0 0,0 18,17V3C18,1.89 17.1,1 16,1M8,23H16V21H8V23Z" />
    </svg>
  }
}
