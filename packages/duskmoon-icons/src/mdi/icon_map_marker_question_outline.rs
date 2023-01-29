#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MapMarkerQuestionOutline)]
pub fn r#icon_map_marker_question_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,1C7.59,1 4,4.59 4,9C4,14.57 10.96,22.34 11.26,22.67L12,23.5L12.74,22.67C13.04,22.34 20,14.57 20,9C20,4.59 16.41,1 12,1M12,20.47C9.82,17.86 6,12.54 6,9A6,6 0 0,1 12,3A6,6 0 0,1 18,9C18,12.83 13.75,18.36 12,20.47M11.13,14H12.88V15.75H11.13M12,5A3.5,3.5 0 0,0 8.5,8.5H10.25A1.75,1.75 0 0,1 12,6.75A1.75,1.75 0 0,1 13.75,8.5C13.75,10.26 11.13,10.04 11.13,12.88H12.88C12.88,10.91 15.5,10.69 15.5,8.5A3.5,3.5 0 0,0 12,5Z" />
    </svg>
  }
}
