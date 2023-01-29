#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DownloadLockOutline)]
pub fn r#icon_download_lock_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22 16A1.08 1.08 0 0 1 23 17V21A1.08 1.08 0 0 1 22 22H17A1.08 1.08 0 0 1 16 21V17A1.08 1.08 0 0 1 17 16V14.5A2.5 2.5 0 0 1 22 14.5V16M21 16V14.5A1.5 1.5 0 0 0 18 14.5V16H21M13 5V11H14.17L12 13.17L9.83 11H11V5H13M15 3H9V9H5L12 16L19 9H15V3M14 18H5V20H14Z" />
    </svg>
  }
}
