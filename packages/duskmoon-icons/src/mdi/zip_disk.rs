#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ZipDisk)]
pub fn r#icon_zip_disk(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7,3L3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V5L17,3V5A1,1 0 0,1 16,6H10A1,1 0 0,1 9,5V3H7M8,10H16A1,1 0 0,1 17,11V19H7V11A1,1 0 0,1 8,10Z" />
    </svg>
  }
}
