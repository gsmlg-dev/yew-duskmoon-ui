#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_HomeEdit)]
pub fn r#icon_home_edit(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 3L2 12H5V20H10V14H14V15.11L19.43 9.68L12 3M21.04 11.14C20.9 11.14 20.76 11.2 20.65 11.3L19.65 12.3L21.7 14.35L22.7 13.35C22.91 13.14 22.91 12.79 22.7 12.58L21.42 11.3C21.32 11.2 21.18 11.14 21.04 11.14M19.06 12.88L13 18.94V21H15.06L21.11 14.93L19.06 12.88Z" />
    </svg>
  }
}
