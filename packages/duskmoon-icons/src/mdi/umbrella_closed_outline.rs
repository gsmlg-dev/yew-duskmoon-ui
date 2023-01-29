#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_UmbrellaClosedOutline)]
pub fn r#icon_umbrella_closed_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 2C12.4 2 12.8 2.2 12.9 2.6L17.5 15H13V19C13 20.7 11.7 22 10 22S7 20.7 7 19V18H9V19C9 19.6 9.4 20 10 20C10.6 20 11 19.6 11 19V15H6.5L11.1 2.6C11.2 2.2 11.6 2 12 2M12 5.9L9.4 13H14.7L12 5.9Z" />
    </svg>
  }
}
