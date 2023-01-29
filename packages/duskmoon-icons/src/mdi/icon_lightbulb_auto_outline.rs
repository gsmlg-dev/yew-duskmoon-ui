#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_LightbulbAutoOutline)]
pub fn r#icon_lightbulb_auto_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 2C12.87 2 16 5.13 16 9C16 11.38 14.81 13.47 13 14.74V17C13 17.55 12.55 18 12 18H6C5.45 18 5 17.55 5 17V14.74C3.19 13.47 2 11.38 2 9C2 5.13 5.13 2 9 2M6 21V20H12V21C12 21.55 11.55 22 11 22H7C6.45 22 6 21.55 6 21M9 4C6.24 4 4 6.24 4 9C4 11.05 5.23 12.81 7 13.58V16H11V13.58C12.77 12.81 14 11.05 14 9C14 6.24 11.76 4 9 4M19 13H17L13.8 22H15.7L16.4 20H19.6L20.3 22H22.2L19 13M16.85 18.65L18 15L19.15 18.65H16.85Z" />
    </svg>
  }
}
