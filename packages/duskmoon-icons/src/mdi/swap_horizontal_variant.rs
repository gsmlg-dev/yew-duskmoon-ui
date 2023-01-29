#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_SwapHorizontalVariant)]
pub fn r#icon_swap_horizontal_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4,6L8,10V7H16A2,2 0 0,1 18,9A2,2 0 0,1 16,11H8A4,4 0 0,0 4,15A4,4 0 0,0 8,19H16V22L20,18L16,14V17H8A2,2 0 0,1 6,15A2,2 0 0,1 8,13H16A4,4 0 0,0 20,9A4,4 0 0,0 16,5H8V2L4,6Z" />
    </svg>
  }
}
