#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SkipPreviousCircleOutline)]
pub fn r#icon_skip_previous_circle_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,2A10,10 0 0,1 22,12A10,10 0 0,1 12,22A10,10 0 0,1 2,12A10,10 0 0,1 12,2M12,4C7.59,4 4,7.59 4,12C4,16.41 7.59,20 12,20C16.41,20 20,16.41 20,12C20,7.59 16.41,4 12,4M16,8V16L11,12M10,8V16H8V8" />
    </svg>
  }
}
