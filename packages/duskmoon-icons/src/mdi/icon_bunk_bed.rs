#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BunkBed)]
pub fn r#icon_bunk_bed(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M1 2V23H3V21H21V23H23V7C23 4.79 21.21 3 19 3H10V8H3V2M6.5 2A2.5 2.5 0 0 0 4 4.5A2.5 2.5 0 0 0 6.5 7A2.5 2.5 0 0 0 9 4.5A2.5 2.5 0 0 0 6.5 2M3 11H21V13.56C20.41 13.21 19.73 13 19 13H10V18H3M6.5 12A2.5 2.5 0 0 0 4 14.5A2.5 2.5 0 0 0 6.5 17A2.5 2.5 0 0 0 9 14.5A2.5 2.5 0 0 0 6.5 12Z" />
    </svg>
  }
}
