#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_PanoramaWideAngle)]
pub fn r#icon_panorama_wide_angle(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 4C8 4 5.2 4.6 3 5C2.5 7 2 8.9 2 12C2 15 2.5 17 3 19C5.2 19.4 8 20 12 20C16 20 18.9 19.4 21 19C21.6 17 22 15 22 12C22 9 21.5 6.9 21 5C18.9 4.6 16 4 12 4Z" />
    </svg>
  }
}
