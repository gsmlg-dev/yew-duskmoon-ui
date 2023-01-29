#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ImageFilterCenterFocusWeak)]
pub fn r#icon_image_filter_center_focus_weak(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5,15H3V19A2,2 0 0,0 5,21H9V19H5M5,5H9V3H5A2,2 0 0,0 3,5V9H5M19,3H15V5H19V9H21V5A2,2 0 0,0 19,3M19,19H15V21H19A2,2 0 0,0 21,19V15H19M12,8A4,4 0 0,0 8,12A4,4 0 0,0 12,16A4,4 0 0,0 16,12A4,4 0 0,0 12,8M12,14A2,2 0 0,1 10,12A2,2 0 0,1 12,10A2,2 0 0,1 14,12A2,2 0 0,1 12,14Z" />
    </svg>
  }
}
