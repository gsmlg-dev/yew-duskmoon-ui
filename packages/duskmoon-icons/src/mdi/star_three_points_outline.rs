#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_StarThreePointsOutline)]
pub fn r#icon_star_three_points_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,9.5L13.2,13.5L16,16.5L12,15.6L7.9,16.5L10.7,13.5L12,9.5M12,2.6L9,12.4L2,19.9L12,17.6L22,20L15,12.5L12,2.6Z" />
    </svg>
  }
}
