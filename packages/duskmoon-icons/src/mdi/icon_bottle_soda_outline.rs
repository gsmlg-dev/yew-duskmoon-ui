#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BottleSodaOutline)]
pub fn r#icon_bottle_soda_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.4 9.58C12.9 7.89 13 4 13 4H14V2H10V4H11S11.1 7.89 9.6 9.58A2 2 0 0 0 9 11V20A2 2 0 0 0 11 22H13A2 2 0 0 0 15 20V11A2 2 0 0 0 14.4 9.58M13 20H11V11L11.1 10.91A6.26 6.26 0 0 0 12 9.5A6.26 6.26 0 0 0 12.9 10.91L13 11Z" />
    </svg>
  }
}
