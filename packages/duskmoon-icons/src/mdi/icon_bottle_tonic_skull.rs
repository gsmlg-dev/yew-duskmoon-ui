#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_BottleTonicSkull)]
pub fn r#icon_bottle_tonic_skull(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 13V22H5V13C5 10.24 7.24 8 10 8V6H9V5H15V6H14V8C16.76 8 19 10.24 19 13M13 4L14 2H10L11 4H13M12 11C9.79 11 8 12.79 8 15C8 16 8.39 16.9 9 17.59V19H10.25V17.5H11.38V19H12.63V17.5H13.75V19H15V17.59C15.61 16.9 16 16 16 15C16 12.79 14.21 11 12 11M10.5 15C9.95 15 9.5 14.55 9.5 14S9.95 13 10.5 13 11.5 13.45 11.5 14 11.05 15 10.5 15M11.25 16.25L12 15L12.75 16.25H11.25M13.5 15C12.95 15 12.5 14.55 12.5 14S12.95 13 13.5 13 14.5 13.45 14.5 14 14.05 15 13.5 15Z" />
    </svg>
  }
}
