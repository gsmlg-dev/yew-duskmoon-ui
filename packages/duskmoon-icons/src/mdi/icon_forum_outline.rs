#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ForumOutline)]
pub fn r#icon_forum_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15,4V11H5.17L4,12.17V4H15M16,2H3A1,1 0 0,0 2,3V17L6,13H16A1,1 0 0,0 17,12V3A1,1 0 0,0 16,2M21,6H19V15H6V17A1,1 0 0,0 7,18H18L22,22V7A1,1 0 0,0 21,6Z" />
    </svg>
  }
}
