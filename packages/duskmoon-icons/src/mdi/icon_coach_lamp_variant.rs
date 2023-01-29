#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CoachLampVariant)]
pub fn r#icon_coach_lamp_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.5 2L12 4L10 6.31L5 9H7L9.5 18L12 20L12.5 22H13.5L14 20L16.5 18L19 9H21L16 6.31L14 4L13.5 2M9 9H17L14.78 17H11.22M3 14V22H11.5L11 20H8L5 17V14Z" />
    </svg>
  }
}
