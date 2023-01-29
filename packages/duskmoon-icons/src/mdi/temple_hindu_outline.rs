#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_TempleHinduOutline)]
pub fn r#icon_temple_hindu_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 11V13H18L15 3V1H13V3H11V1H9V3.1L6 13H4V11H2V22H11V17H13V22H22V11H20M15.3 11H8.7L9.3 9H14.7L15.3 11M14.1 7H9.9L10.5 5H13.5L14.1 7M20 20H15V15H9V20H4V15H7.5L8.1 13H15.9L16.5 15H20V20Z" />
    </svg>
  }
}
