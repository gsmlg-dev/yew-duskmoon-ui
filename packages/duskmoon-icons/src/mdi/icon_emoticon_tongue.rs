#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_EmoticonTongue)]
pub fn r#icon_emoticon_tongue(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M7.27,11C7.1,10.71 7,10.36 7,10C7,8.89 7.89,8 9,8A2,2 0 0,1 11,10C11,10.36 10.9,10.71 10.73,11C10.39,10.4 9.74,10 9,10C8.26,10 7.61,10.4 7.27,11M16,15H15C15,17 14.1,18 13,18C11.9,18 11,17 11,15H8V13H16V15M16.73,11C16.39,10.4 15.74,10 15,10C14.26,10 13.61,10.4 13.27,11C13.1,10.71 13,10.36 13,10C13,8.89 13.89,8 15,8A2,2 0 0,1 17,10C17,10.36 16.9,10.71 16.73,11Z" />
    </svg>
  }
}
