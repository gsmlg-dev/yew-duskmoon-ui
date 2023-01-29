#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_FolderOff)]
pub fn r#icon_folder_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.11 21.46L20.84 22.73L18.11 20H4C2.9 20 2 19.11 2 18V6C2 5.42 2.25 4.9 2.64 4.53L1.11 3L2.39 1.73L22.11 21.46M22 18V8C22 6.89 21.1 6 20 6H12L10 4H7.2L21.88 18.68C21.96 18.47 22 18.24 22 18Z" />
    </svg>
  }
}
