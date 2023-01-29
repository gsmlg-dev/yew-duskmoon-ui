#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TransferUp)]
pub fn r#icon_transfer_up(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8,21V19H16V21H8M8,17V15H16V17H8M8,13V11H16V13H8M19,9H5L12,2L19,9Z" />
    </svg>
  }
}
