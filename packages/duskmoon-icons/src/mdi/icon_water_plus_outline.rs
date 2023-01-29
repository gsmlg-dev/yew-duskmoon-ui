#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_WaterPlusOutline)]
pub fn r#icon_water_plus_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 18C12 18.7 12.12 19.36 12.34 20C12.23 20 12.12 20 12 20C8.69 20 6 17.31 6 14C6 10 12 3.25 12 3.25S16.31 8.1 17.62 12C16.93 12.06 16.28 12.22 15.67 12.47C15 10.68 13.5 8.33 12 6.39C10 8.96 8 12.23 8 14C8 16.21 9.79 18 12 18M19 17V14H17V17H14V19H17V22H19V19H22V17H19Z" />
    </svg>
  }
}
