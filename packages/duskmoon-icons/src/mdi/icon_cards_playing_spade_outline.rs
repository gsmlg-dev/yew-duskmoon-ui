#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CardsPlayingSpadeOutline)]
pub fn r#icon_cards_playing_spade_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17 2H7C5.9 2 5 2.9 5 4V20C5 21.1 5.9 22 7 22H17C18.1 22 19 21.1 19 20V4C19 2.9 18.1 2 17 2M17 20H7V4H17V20M12 7.7L11.4 8.2C9.4 10.1 8 11.3 8 12.8C8 14 9 15 10.2 15C10.6 15 11 14.9 11.4 14.7L10.5 17H13.5L12.6 14.7C12.9 14.9 13.4 15 13.8 15C15 15 16 14.1 16 12.8C16 11.3 14.6 10.1 12.6 8.2L12 7.7Z" />
    </svg>
  }
}
