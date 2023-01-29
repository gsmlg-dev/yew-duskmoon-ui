#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Wardrobe)]
pub fn r#icon_wardrobe(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4 4V19C4 20.1 4.9 21 6 21V22H8V21H11.5V2H6C4.9 2 4 2.9 4 4M8 10H10V13H8V10M18 2H12.5V21H16V22H18V21C19.1 21 20 20.1 20 19V4C20 2.9 19.1 2 18 2M16 13H14V10H16V13Z" />
    </svg>
  }
}
