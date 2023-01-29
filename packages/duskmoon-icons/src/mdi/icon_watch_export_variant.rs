#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_WatchExportVariant)]
pub fn r#icon_watch_export_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14,11H19L16.5,8.5L17.92,7.08L22.84,12L17.92,16.92L16.5,15.5L19,13H14V11M8,0H16L16.83,5H17C17.28,5 17.54,5.06 17.78,5.16L15.94,7H7V17H15.94L17.78,18.84C17.54,18.94 17.28,19 17,19H16.83L16,24H8L7.17,19H7A2,2 0 0,1 5,17V7C5,5.89 5.9,5 7,5H7.17L8,0Z" />
    </svg>
  }
}
