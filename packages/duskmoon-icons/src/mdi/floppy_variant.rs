#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FloppyVariant)]
pub fn r#icon_floppy_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3,3V21H21V3H3M12,10A2,2 0 0,1 14,12A2,2 0 0,1 12,14A2,2 0 0,1 10,12A2,2 0 0,1 12,10M12,15A1,1 0 0,1 13,16V19A1,1 0 0,1 12,20A1,1 0 0,1 11,19V16A1,1 0 0,1 12,15Z" />
    </svg>
  }
}
