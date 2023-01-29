#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Nature)]
pub fn r#icon_nature(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13,16.12C16.47,15.71 19.17,12.76 19.17,9.17C19.17,5.3 16.04,2.17 12.17,2.17A7,7 0 0,0 5.17,9.17C5.17,12.64 7.69,15.5 11,16.06V20H5V22H19V20H13V16.12Z" />
    </svg>
  }
}
