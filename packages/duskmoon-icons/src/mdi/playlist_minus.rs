#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_PlaylistMinus)]
pub fn r#icon_playlist_minus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 16H10V14H3M12 14V16H22V14M14 6H3V8H14M14 10H3V12H14V10Z" />
    </svg>
  }
}
